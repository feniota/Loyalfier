use image::RgbaImage;
use photon_rs::{multiple, transform, PhotonImage};
use rand::Rng;
use std::{cell::Cell, collections::HashMap, rc::Rc};

use super::arranger::{Sample, Table};

/// 存储一张纸上的样本  
///   
/// - border：表格区域边框四角的坐标
/// - unit_size：单元格的大小
#[derive(Clone)]
pub struct Paper {
    img_size: [usize; 2],
    pub border: [[usize; 2]; 4], // 没准有用呢
    pub unit_size: [usize; 2],
    table: Vec<Cell<Transform>>,
    samples: HashMap<usize, Rc<PhotonImage>>,
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
    /// 由纸张尺寸生成图片尺寸
    pub fn pixels(&self) -> [usize; 2] {
        match self {
            PaperSize::A4 => [2480, 3508],
            PaperSize::A5 => [1240, 1754],
            PaperSize::B5 => [1624, 1075],
        }
    }
}

/// 变换（定义一张样本图片在纸张图片上的位置等）
///
/// - id: 这个变换指向的样本图片在哈希表内的键
/// - position：样本图片的**中心**坐标
/// - rotation：旋转（单位为度）
/// - scale：缩放比例
/// - row：这个变换在纸张表格上的行
/// - column：这个变换在纸张表格上的列
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
    /// 随机化所有单元格  
    /// 为了获得最好效果，建议在 obfuscate 之前调用
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

    /// 混淆（将某一行上斜/下斜，使用对数曲线）  
    /// 为了获得最好效果，建议在 alter 之前调用，系数尽可能大  
    ///  
    /// 参数：  
    /// - obfuscation: 混淆方式
    /// - row: 行号
    /// - coefficient: 系数（对数曲线垂直方向伸长）
    pub fn obfuscate(&self, obfuscation: PaperObfuscation, row: usize, coefficient: f32) -> () {
        //const COEFFICIENT: f32 = 20.0; // 系数（对数曲线垂直方向伸长）
        const HORIZONTAL_SHIFT: f32 = 1.0; //对数曲线水平位移
        let mut rng = rand::thread_rng(); // 当前线程的 Rng（随机数生成用）
        for i in self.table.iter() {
            let mut current = i.get(); // 获取当前单元格内的 Transform 元素
            if current.row != row {
                // 只处理 {row} 行的单元格
                continue;
            }
            let entropy = match obfuscation {
                PaperObfuscation::Upward => rng.gen_range(0.80..=1.00),
                PaperObfuscation::Downward => rng.gen_range(-0.80..=-1.00),
            }; // 熵

            // 计算纵坐标的变化量
            let delta_y: i32 =
                (coefficient * entropy * f32::ln_1p(current.column as f32 + HORIZONTAL_SHIFT))
                    as i32;

            // 计算旋转角度的变化量
            let delta_deg: f32 =
                coefficient * entropy * f32::atan(1.0 / (current.column as f32 + HORIZONTAL_SHIFT));

            current.position[1] = i32_to_usize(current.position[1] as i32 - delta_y);
            current.rotation -= delta_deg; // 正角度代表顺时针旋转，因此这里是减

            // 回传值
            i.set(current);
        }
    }

    /// 生成图片
    pub fn make_image(&self) -> PhotonImage {
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

            // 缩放
            transformed = transform::resize(
                &transformed,
                (current_img.get_width() as f32 * current.scale).round() as u32,
                (current_img.get_height() as f32 * current.scale).round() as u32,
                transform::SamplingFilter::Lanczos3,
            );

            // 旋转
            // 实践证明，如果传递给 rotate 一个负数角度，最终图片会出现一个奇怪的平移，导致结果像是酒醉了一样（
            // 这里通过翻转后旋转来实现逆时针效果
            if current.rotation >= 0.0 {
                transformed = transform::rotate(&transformed, current.rotation);
            } else {
                transform::fliph(&mut transformed);
                transformed = transform::rotate(&transformed, current.rotation.abs());
                transform::fliph(&mut transformed);
            }

            // 插入当前样本图片
            let half_height = transformed.get_height().wrapping_div(2) as usize;
            let half_width = transformed.get_width().wrapping_div(2) as usize;
            multiple::watermark(
                &mut img,
                &transformed,
                current.position[0].wrapping_sub(half_width) as i64,
                current.position[1].wrapping_sub(half_height) as i64,
            )
        }
        img
    }

    /// 由 arranger 部分填好的 Table 生成 Paper  
    ///  
    /// 参数：  
    /// - samples: 所有样本的 PhotonImage
    /// - table: 填好的 Table
    /// - page_index: 当前纸张所在的页码
    /// - paper_size: 纸张大小（可以用 PaperSize::pixels 快捷生成）
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
