use crate::func::arranger::{make_table, Sample};

#[test]
pub fn test() {
    //测试配置
    let rows = 3; //行
    let columns = 3; //列
    let pages = 3; //页

    //样本生成
    let mut samples = vec![];
    for i in 0..(pages * 4) {
        samples.push(Sample {
            id: i,
            dummy: false,
        });
    }
    let table = make_table(samples, pages, rows, columns);
    match table {
        Ok(x) => {
            assert_eq!(
                x.len(),
                rows + 2,
                "行数错误，预期为{}，实际为{}",
                rows + 2,
                x.len()
            );
            assert_eq!(
                x[0].len(),
                columns + 1,
                "列数错误，预期为{}，实际为{}",
                columns + 1,
                x[0].len()
            );
            assert_eq!(
                x[0][0].len(),
                pages,
                "页数错误，预期为{}，实际为{}",
                pages,
                x[0][0].len()
            );
            assert_ne!(x[1][1][0], Sample { id: 0, dummy: true });
            println!("converted vector:\n{:?}]", x)
        }
        Err(x) => {
            assert!(false, "Encountered unexpected error {:?}", x)
        }
    };
}
