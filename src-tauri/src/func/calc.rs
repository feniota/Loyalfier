use std::{cell::Cell, collections::HashMap};

use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Sample {
    id: u16,
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

#[derive(Debug)]
pub struct Table {
    table: HashMap<[u16; 3], Sample>,
    length: u16,
    width: u16,
    height: u16,
}

pub enum MakeTableError {
    NotEnoughSamples,
    InvalidParameter,
    Other,
}

// The given sample list should be sorted in ascending order, and the data's id field should be equal to its index in the vector
// The resulting 3D vector is arranged as [z,x,y], starting with [0,1,1]
pub fn make_table(samples: Vec<Sample>, z: u16, x: u16, y: u16) -> Result<Table, MakeTableError> {
    if x == 0 || y == 0 || z == 0 || samples.is_empty() {
        return Err(MakeTableError::InvalidParameter);
    } else if samples.len() < z.into() || samples.len() < 9 {
        return Err(MakeTableError::NotEnoughSamples);
    }

    let mut table: HashMap<[u16; 3], Sample> = HashMap::new();
    for i in 0..z {
        for ii in 0..=x {
            table.insert([i, ii, 0], Sample { id: 0, dummy: true });
        }
        for ii in 1..=y {
            table.insert([i, 0, ii], Sample { id: 0, dummy: true });
        }
    }
    let current: Cell<[u16; 3]> = Cell::new([0, 1, 1]);
    let mut rng = rand::thread_rng();
    let next = || {
        let cur = current.get();
        if cur[1] == x {
            current.set([cur[0], 1, cur[2] + 1]);
        } else {
            current.set([cur[0], cur[1] + 1, cur[2]]);
        }
    };
    let empty_sample = Sample { id: 0, dummy: true };
    loop {
        let cur = current.get();
        if table.contains_key(&cur) {
            next();
            continue;
        }
        'inner: loop {
            let id = rng.gen_range(0..samples.len());
            let sample = samples[id];
            if *table
                .get(&[cur[0], cur[1] - 1, cur[2] - 1])
                .unwrap_or(&empty_sample)
                == sample
                || *table
                    .get(&[cur[0], cur[1], cur[2] - 1])
                    .unwrap_or(&empty_sample)
                    == sample
                || *table
                    .get(&[cur[0], cur[1] + 1, cur[2] - 1])
                    .unwrap_or(&empty_sample)
                    == sample
                || *table
                    .get(&[cur[0], cur[1] - 1, cur[2]])
                    .unwrap_or(&empty_sample)
                    == sample
            {
                continue 'inner;
            }
            table.insert(cur, sample);
            break 'inner;
        }
        if cur[1] == x && cur[2] == y {
            break;
        }
        next();
    }
    for i in 1..z {
        current.set([i, 1, 1]);
        'inner: loop {
            let sample_max = samples.len() - 1;
            let cur = current.get();
            let before = *table
                .get(&[cur[0] - 1, cur[1], cur[2]])
                .unwrap_or(&empty_sample);
            if usize::from(before.id) == sample_max {
                table.insert(cur, samples[0]);
            } else {
                table.insert(cur, samples[usize::from(before.id + 1)]);
            }
            if cur[1] == x && cur[2] == y {
                break 'inner;
            }
            next();
        }
    }

    Ok(Table {
        table,
        length: x,
        width: y,
        height: z,
    })
}

pub fn generate_vector(table: Table) -> Vec<Vec<Vec<Sample>>> {
    let null_smp = Sample { id: 0, dummy: true };
    /*    let mut a: Vec<Vec<Vec<Sample>>> =
    vec![
        vec!(vec!(null_smp; usize::from(table.width)); usize::from(table.length));
        usize::from(table.height)
    ];*/
    let mut a: Vec<Vec<Vec<Sample>>> = vec![];
    for i in 0..table.height {
        let empty_page: Vec<Vec<Sample>> = vec![];
        a.push(empty_page);
        for ii in 0..table.length {
            let empty_line: Vec<Sample> = vec![];
            a[usize::from(i)].push(empty_line);
            'inner: for iii in 0..table.width {
                let key: [u16; 3] = [i, ii + 1, iii + 1];
                if !table.table.contains_key(&key) {
                    continue 'inner;
                }
                let sample = *table.table.get(&key).unwrap_or(&null_smp);
                a[usize::from(i)][usize::from(ii)].push(sample);
            }
        }
    }
    a
}

pub fn test() {
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
        ],
        3,
        3,
        3,
    );
    match (table) {
        Ok(x) => {
            println!("{:?}", x.table);
            let vec = generate_vector(x);
            println!("converted vector:\n{:?}]", vec);
        }
        _ => {}
    };
}
