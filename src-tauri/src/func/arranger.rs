use rand::{seq::SliceRandom, thread_rng};

#[derive(Clone, Copy, Debug)]
pub struct Sample {
    id: usize,
    dummy: bool,
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
pub enum MakeTableError {
    NotEnoughSamples,
    InvalidParameter,
    Other,
}

pub fn make_table(
    samples: Vec<Sample>,
    pages: usize,
    rows: usize,
    columns: usize,
) -> Result<Vec<Vec<Vec<Sample>>>, MakeTableError> {
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
    for x in 1..(rows + 1) {
        for y in 1..(columns + 1) {
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
    Ok(table)
}

#[test]
pub fn test() {
    let Rows = 3;
    let Columns = 3;
    let Pages = 3;
    let table = make_table(
        vec![
            Sample {
                id: 0,
                dummy: false,
            },
            Sample {
                id: 1,
                dummy: false,
            },
            Sample {
                id: 2,
                dummy: false,
            },
            Sample {
                id: 3,
                dummy: false,
            },
            Sample {
                id: 4,
                dummy: false,
            },
            Sample {
                id: 5,
                dummy: false,
            },
            Sample {
                id: 6,
                dummy: false,
            },
            Sample {
                id: 7,
                dummy: false,
            },
            Sample {
                id: 8,
                dummy: false,
            },
            Sample {
                id: 9,
                dummy: false,
            },
            Sample {
                id: 10,
                dummy: false,
            },
            Sample {
                id: 11,
                dummy: false,
            },
            Sample {
                id: 12,
                dummy: false,
            },
        ],
        Pages,
        Rows,
        Columns,
    );
    match table {
        Ok(x) => {
            assert_eq!(x.len(), Rows + 2);
            assert_eq!(x[0].len(), Columns + 1);
            assert_eq!(x[0][0].len(), Pages);
            assert_ne!(x[1][1][0], Sample { id: 0, dummy: true });
            println!("converted vector:\n{:?}]", x)
        }
        _ => {}
    };
}
