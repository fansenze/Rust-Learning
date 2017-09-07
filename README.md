# Rust Learning  

### 本项目目录 

**通用概念**  
> [variables_mutability ── 变量和可变性](https://github.com/fansenze/Rust-Learning/tree/master/variables_mutability)  
> [data_types ── 数据类型](https://github.com/fansenze/Rust-Learning/tree/master/data_types)  
> [how_funtions_work ── 函数是如何工作的](https://github.com/fansenze/Rust-Learning/tree/master/how_funtions_work)  
> [control_flow ── 控制流](https://github.com/fansenze/Rust-Learning/tree/master/control_flow)  

**所有权**



## Cargo: Rust 的构建系统和包管理工具  
  

### 1. 创建
  **cargo new myProject `--bin`** （bin表示创建二进制项目）  
  
  + `--vcs` （其他版本控制系统）  
  
  
  创建完成后的目录结构:  

  ```lib
  myProject  
      ├──── src/  
      │      ├──── main.rs     # 入口文件
      ├──── target/  
      │      ├──── debug/      # 开发环境文件存放目录
      │      ├──── release/    # 生产环境文件存放目录 (初始化时，不会生成，只有跑过 cargo build/run --release 之后才会生成)  
      ├──── Cargo.lock         # 项目依赖描述，Cargo会自动处理该文件，无视它就好
      ├──── Cargo.toml         # 项目描述，类似package.json  
      ├──── .gitignore         # 这个不需要我介绍了吧  
  ```

### 2. 编译运行
  **cargo build**   `编译rust文件，生成二进制可执行文件，通常叫做二进制文件(binaries)`  

  **cargo run**     `编译并运行rust文件`  

  + build 和 run 命令不带参数则编译后的文件生成到 `target/debug` 文件夹中,快速重新构建满足开发环境  
  + build 和 run 命令带上 `--release` 则优化编译项目,生成到 `target/release` 文件中,用作生产环境  
  + 不用 **rustc** 命令是因为任何平台(Windows / OSX / Linux)中，Cargo 的命令都相同，另外 Cargo 会自动记录程序依赖到cargo.lock中

  
  
参考文档: [Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn)  

`本项目纯属自学项目`
