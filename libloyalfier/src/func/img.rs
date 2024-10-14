use rand::Rng;
use ril::{Image, Rgba};
use std::{cell::Cell, collections::HashMap, rc::Rc};

use super::arranger::{Sample, Table};

#[derive(Clone)]
pub struct Paper {
    pub img_size: [usize; 2],
    pub border: [[usize; 2]; 4],
    pub unit_size: [usize; 2],
    pub table: Vec<Cell<Transform>>,
    pub samples: HashMap<usize, Rc<Image<Rgba>>>,
}

pub enum PaperObfuscation {
    Upward,
    Downward,
}

pub enum PaperSize {
    A4,
    A5,
    B5,
}
impl PaperSize {
    pub fn pixels(&self) -> [usize; 2] {
        match self {
            PaperSize::A4 => [2480, 3508],
            PaperSize::A5 => [1240, 1754],
            PaperSize::B5 => [1624, 1075],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub id: usize,
    pub position: [usize; 2],
    // Take integer here, not float, since ril use degrees for rotation
    pub rotation: i32,
    pub scale: f32,
    pub row: usize,
    pub column: usize,
}

impl Paper {
    pub fn alter(&self) -> () {
        let mut rng = rand::thread_rng();
        for t in self.table.iter() {
            let mut transform = t.get();
            // 每个样本随机旋转一个小角度
            transform.rotation = rng.gen_range(-3..=3);
            // 每个样本随机偏移 +-3mm
            transform.position = [
                i32_to_usize(transform.position[0] as i32 + rng.gen_range(-11..=11)),
                i32_to_usize(transform.position[1] as i32 + rng.gen_range(-11..=11)),
            ];
            // 每个样本随机缩放一小点点
            transform.scale = rng.gen_range(0.97..=1.03);
            // 回传值
            t.set(transform);
        }
        // 因为样本本身是在 Cell 里的，所以 self 本身已经是变过的了，这里不用再传出改变后的值
    }

    pub fn obfuscate(&self, obfuscation: PaperObfuscation, row: usize) -> () {
        const COEFFICIENT: f32 = 0.0005;
        const HORIZONTAL_SHIFT: f32 = 2.0; //对数曲线水平位移
        let mut rng = rand::thread_rng();
        let phase: i32 = rng.gen_range(-18..=18); // 相位，将此行整体上移或下移 +=5mm
        for i in self.table.iter() {
            let mut current = i.get();
            if current.row != row {
                continue;
            }
            let entropy = rng.gen_range(0.00..=1.00);
            let delta_y: i32 =
                (COEFFICIENT * entropy * f32::ln_1p(current.column as f32 + HORIZONTAL_SHIFT))
                    as i32;
            let delta_deg: i32 = f32::atan(current.column as f32 + HORIZONTAL_SHIFT) as i32;
            match obfuscation {
                PaperObfuscation::Upward => {
                    current.position[1] += i32_to_usize(delta_y + phase);
                    current.rotation += delta_deg;
                }
                PaperObfuscation::Downward => {
                    current.position[1] =
                        i32_to_usize(current.position[1] as i32 + phase - delta_y);
                    current.rotation -= delta_deg;
                }
            }
            i.set(current);
        }
    }

    pub fn make_image(&self) -> Image<Rgba> {
        let mut img = Image::new(
            self.img_size[0] as u32,
            self.img_size[1] as u32,
            Rgba::transparent(),
        );
        for t in self.table.iter() {
            let current: Transform = t.get();
            let current_img = self.samples.get(&current.id).unwrap();
            let transformed = Image::clone(current_img) //.rotated(current.rotation)
                .resized(
                    (current_img.width() as f32 * current.scale).round() as u32,
                    (current_img.height() as f32 * current.scale).round() as u32,
                    ril::ResizeAlgorithm::Lanczos3,
                );
            img.paste(
                current.position[0] as u32,
                current.position[1] as u32,
                &transformed,
            );
        }
        img
    }

    pub fn make(
        samples: HashMap<Sample, Rc<Image<Rgba>>>,
        table: Table,
        page_index: usize,
        paper_size: [usize; 2],
    ) -> Self {
        // Convert hashmap to avoid Sample to be used outside the algorithm part
        let mut map: HashMap<usize, Rc<Image<Rgba>>> = HashMap::new();
        for (key, value) in samples.iter() {
            let id = key.id;
            let img = value.clone();
            map.insert(id, img);
        }

        // Calculate necessary data given table and paper size
        let border = [
            [38, 38],
            [paper_size[0].wrapping_sub(38), 38],
            [38, paper_size[1].wrapping_sub(38)],
            [
                paper_size[0].wrapping_sub(38),
                paper_size[1].wrapping_sub(38),
            ],
        ];
        let unit_size = [
            (border[1][0].wrapping_sub(border[0][0])).wrapping_div(table.columns),
            (border[3][1].wrapping_sub(border[0][1])).wrapping_div(table.rows),
        ];
        let unit_half = [unit_size[0].wrapping_div(2), unit_size[1].wrapping_div(2)];
        let mut transforms: Vec<Cell<Transform>> = vec![];
        for x in 1..=table.rows {
            'inner: for y in 1..=table.columns {
                let z = page_index;
                let current = table.table[x][y][z];
                if current.dummy {
                    continue 'inner;
                }
                let position = [
                    border[0][0]
                        .wrapping_add(unit_half[0])
                        .wrapping_add((y - 1).wrapping_mul(unit_size[0])),
                    border[0][1]
                        .wrapping_add(unit_half[1])
                        .wrapping_add((x - 1).wrapping_mul(unit_size[1])),
                ];
                let t = Transform {
                    id: current.id,
                    position,
                    rotation: 0,
                    scale: 1.0,
                    row: x,
                    column: y,
                };
                transforms.push(Cell::new(t));
            }
        }

        Paper {
            img_size: paper_size,
            border,
            unit_size,
            table: transforms,
            samples: map,
        }
    }
}
fn i32_to_usize(x: i32) -> usize {
    if x < 0 {
        0
    } else {
        x.abs() as usize
    }
}
