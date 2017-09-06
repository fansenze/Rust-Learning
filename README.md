## Cargo: rust 的构建系统和包管理工具

> ### cargo new xxxx —bin （会初始化git仓库，以及.gitignore文件, bin表示创建二进制项目）
+ —vsc （其他版本控制系统）

> ### cargo build  // 编译rust文件，生成二进制可执行文件，通常叫做二进制文件
> ### cargo run   // 编译并运行rust文件
+ build和run命令不带参数则编译后的文件生成在debug文件中,为了开发则需要快速重新构建。
+ build和run 命令带上 —release 则优化编译项目生成到release文件中,用作生产环境
+ 不用rustc命令是因为任何平台 cargo的命令都相同，而且cargo会自动记录程序以来到cargo.lock中
