use image::RgbaImage;
use photon_rs::{multiple, transform, PhotonImage};
use rand::Rng;
use std::{cell::Cell, collections::HashMap, rc::Rc};

use super::arranger::{Sample, Table};

#[derive(Clone)]
pub struct Paper {
    pub img_size: [usize; 2],
    pub border: [[usize; 2]; 4],
    pub unit_size: [usize; 2],
    pub table: Vec<Cell<Transform>>,
    pub samples: HashMap<usize, Rc<PhotonImage>>,
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
#[derive(Debug)]
pub enum MakePaperError {
    PageOutOfIndex,
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
    pub rotation: f32,
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
            transform.rotation = rng.gen_range(-3.0..=3.0);
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
        const COEFFICIENT: f32 = 15.0;
        const HORIZONTAL_SHIFT: f32 = 0.0; //对数曲线水平位移
        let mut rng = rand::thread_rng();
        // let phase: i32 = rng.gen_range(-18..=18); // 相位（弃用），将此行整体上移或下移 +=5mm
        for i in self.table.iter() {
            let mut current = i.get();
            if current.row != row {
                continue;
            }
            let entropy = rng.gen_range(0.70..=1.00);
            let delta_y: i32 =
                (COEFFICIENT * entropy * f32::ln_1p(current.column as f32 + HORIZONTAL_SHIFT))
                    as i32;
            let delta_deg: f32 = f32::atan(1_f32 / (current.column as f32 + HORIZONTAL_SHIFT));
            match obfuscation {
                PaperObfuscation::Upward => {
                    //current.position[1] += i32_to_usize(delta_y + phase);
                    current.position[1] += i32_to_usize(delta_y);
                    current.rotation += delta_deg;
                }
                PaperObfuscation::Downward => {
                    current.position[1] =
                        //i32_to_usize(current.position[1] as i32 + phase - delta_y);
                        i32_to_usize(current.position[1] as i32 - delta_y);
                    current.rotation -= delta_deg;
                }
            }
            i.set(current);
        }
    }

    pub fn make_image(&self) -> PhotonImage {
        /*let mut img = Image::new(
            self.img_size[0] as u32,
            self.img_size[1] as u32,
            Rgba::transparent(),
        );*/
        let img_buffer = RgbaImage::new(self.img_size[0] as u32, self.img_size[1] as u32);
        let mut img: PhotonImage = PhotonImage::new(
            img_buffer.into_raw(),
            self.img_size[0] as u32,
            self.img_size[1] as u32,
        );
        for t in self.table.iter() {
            let current: Transform = t.get();
            let current_img = self.samples.get(&current.id).unwrap();
            let mut transformed = PhotonImage::clone(current_img);
            transformed = transform::resize(
                &transformed,
                (current_img.get_width() as f32 * current.scale).round() as u32,
                (current_img.get_height() as f32 * current.scale).round() as u32,
                transform::SamplingFilter::Lanczos3,
            );
            // 实践证明，如果传递给 rotate 一个负数角度，最终图片会出现一个奇怪的平移，导致结果像是酒醉了一样（
            // 这里通过翻转后旋转来实现逆时针效果
            if current.rotation >= 0.0 {
                transformed = transform::rotate(&transformed, current.rotation);
            } else {
                transform::fliph(&mut transformed);
                transformed = transform::rotate(&transformed, current.rotation.abs());
                transform::fliph(&mut transformed);
            }
            let half_height = transformed.get_height().wrapping_div(2) as usize;
            let half_width = transformed.get_width().wrapping_div(2) as usize;
            /*img.paste(
                current.position[0].wrapping_sub(half_width) as u32,
                current.position[1].wrapping_sub(half_height) as u32,
                &transformed,
            );*/
            multiple::watermark(
                &mut img,
                &transformed,
                current.position[0].wrapping_sub(half_width) as i64,
                current.position[1].wrapping_sub(half_height) as i64,
            )
        }
        img
    }

    pub fn make(
        samples: HashMap<Sample, Rc<PhotonImage>>,
        table: Table,
        page_index: usize,
        paper_size: [usize; 2],
    ) -> Result<Self, MakePaperError> {
        if page_index >= table.pages {
            return Err(MakePaperError::PageOutOfIndex);
        }
        // Convert hashmap to avoid Sample to be used outside the algorithm part
        let mut map: HashMap<usize, Rc<PhotonImage>> = HashMap::new();
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
            (border[3][0].wrapping_sub(border[0][0])).wrapping_div(table.columns),
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
                    rotation: 0.0,
                    scale: 1.0,
                    row: x,
                    column: y,
                };
                transforms.push(Cell::new(t));
            }
        }

        Ok(Paper {
            img_size: paper_size,
            border,
            unit_size,
            table: transforms,
            samples: map,
        })
    }
}
fn i32_to_usize(x: i32) -> usize {
    if x < 0 {
        0
    } else {
        x.abs() as usize
    }
}
