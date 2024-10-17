# Loyalifier

忠诚化器，使你忠于老师的重度作业

>愿你永远不会用上这个软件

此项目暂时未完成，但已可用，修改[测试项目]（/loyalfier_test)中的`src/main.rs`并运行`cargo run --release`

```
    // Configuration field
    const SAMPLES: usize = 20;                  //样本数量
    const COLUMNS: usize = 4;                   //单页列数
    const ROWS: usize = 12;                     //单页行数
    const PAGES: usize = 5;                     //页数
    let paper_size = PaperSize::B5.pixels();    //输出纸张大小，支持A4,A5,B5
    let in_path = String::from(".\\images_\\"); //样本目录
    let out_path = String::from(".\\output\\"); //输出目录
```

应对重度罚抄时，你需要抄好一定的次数（大于9且大于总页数*4），对扫描的样本分片为合适且相同的大小，放到样本目录，运行程序。

原理为随机排列并随机缩放/位移/旋转变换样本，混淆后用喷墨打印机打印即可混过老师的检查。

# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
