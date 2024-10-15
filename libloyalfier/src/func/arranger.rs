use rand::{seq::SliceRandom, thread_rng};

/// 样本  
/// - id: usize 样本编号（必须唯一）
/// - dummy: bool 是否为“假数据”（一般应为 false）
#[derive(Clone, Copy, Debug, Eq, Hash)]
pub struct Sample {
    pub id: usize,
    pub dummy: bool,
}
impl PartialEq for Sample {
    fn eq(&self, other: &Self) -> bool {
        if self.dummy || other.dummy {
            false
        } else {
            self.id == other.id
        }
    }
}

/// 表格  
/// - rows: 行数
/// - columns: 列数
/// - pages: 纸张数
#[derive(Clone)]
pub struct Table {
    pub rows: usize,
    pub columns: usize,
    pub pages: usize,
    pub table: Vec<Vec<Vec<Sample>>>,
}
#[derive(Debug)]
pub enum MakeTableError {
    NotEnoughSamples,
    InvalidParameter,
    Other,
}

/// 生成表格  
///   
/// 参数：  
/// - samples: `Vec<Sample>` 样本（传入的值 dummy 必须为 false）  
/// - pages: usize 页面数量  
/// - rows: usize 行数  
/// - columns: usize 列数
pub fn make_table(
    samples: Vec<Sample>,
    pages: usize,
    rows: usize,
    columns: usize,
) -> Result<Table, MakeTableError> {
    if rows == 0 || columns == 0 || pages == 0 || samples.is_empty() {
        //判断行列页是否为0，以及Sample是否为空
        return Err(MakeTableError::InvalidParameter);
    } else if samples.len() < (pages * 4) || samples.len() < 9 {
        //判断样本是否充足
        return Err(MakeTableError::NotEnoughSamples);
    }

    //创建三维列表，由外至内分别为行，列，页
    let mut table: Vec<Vec<Vec<Sample>>> =
        vec![vec![vec![Sample { id: 0, dummy: true }; pages]; columns + 1]; rows + 2];

    //初始化Rand
    let mut rng = thread_rng();

    //填充列表
    for x in 1..=rows {
        for y in 1..=columns {
            for z in 0..pages {
                let minusion = samples
                    .iter()
                    .filter(|&u| {
                        ![table[x - 1][y][z]].contains(u)//排除左边元素
                            && ![table[x][y - 1][z]].contains(u)//排除上边元素
                            && ![table[x - 1][y - 1][z]].contains(u)//排除左上元素
                            && ![table[x + 1][y - 1][z]].contains(u)//排除右上元素
                            && !table[x][y].contains(u) //排除同位置元素
                    })
                    .collect::<Vec<_>>();
                table[x][y].remove(z); //移除当前位置空元素，防止Vector扩大造成开销
                table[x][y].insert(z, **minusion.choose(&mut rng).unwrap()); //填入当前元素
            }
        }
    }
    Ok(Table {
        rows: rows,
        columns: columns,
        pages: pages,
        table: table,
    })
}
