use crate::func::arranger::{Sample, make_table};

#[test]
pub fn test() {
    let rows = 3;
    let columns = 3;
    let pages = 3;
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
        pages,
        rows,
        columns,
    );
    match table {
        Ok(x) => {
            assert_eq!(x.len(), rows + 2);
            assert_eq!(x[0].len(), columns + 1);
            assert_eq!(x[0][0].len(), pages);
            assert_ne!(x[1][1][0], Sample { id: 0, dummy: true });
            println!("converted vector:\n{:?}]", x)
        }
        _ => {}
    };
}