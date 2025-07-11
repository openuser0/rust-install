//! #函数操作 模块 ( function_mod_https.rs )

//! ##功能

//! 封装函数操作

//! ##作者

//! songjiaqicode

//! ##初始化日期

//! 2025.7.11

/* 引入标准库 */

use std::io::stdin;
use std::path::{PathBuf, Path};
use std::env::var;
use std::process::Stdio;
use tokio::fs::{OpenOptions};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::process::Command;

/* 引入私有库 */



/* 添加 cargo 镜像<1.68版本以上> */
pub async fn cargo() -> Result<(), Box<dyn std::error::Error>> {
    /* 定义 .cargo 路径 */
    let cargo_path = res_path(".cargo/config.toml");

    /* 镜像存在性判断 */
    if let Ok(mut e) = OpenOptions::new().read(true).open(&cargo_path).await {
        let mut buf = String::new();
        e.read_to_string(&mut buf).await?;

        /* 定义镜像关键部分 */
        let https = vec![
            "[source.crates-io]",
            "replace-with = 'mirror'",
            "[source.mirror]",
            r#"registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/""#,
            "[registries.mirror]",
            r#"index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/""#
        ];

        /* 镜像存在性判断 */
        let mut all = true;
        for i in https { if !buf.contains(i) { all = false; break; }  }
        if all { println!("cargo 镜像已存在,无需添加"); std::process::exit(0); }
    };

    /* 确认rust版本 */
    println!("\n如果你的rust版本低于1.68,或者未安装rust,切勿添加镜像,否则会报错☠️\n\n执行 cargo -V&&rustc -V 命令来确认 rust 版本以及是否安装");
    println!("\n你是否已安装 rust 并且版本大于或等于 1.68? [y/n]");
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    if buf.trim() == "y" { () }else { println!("已取消镜像添加"); std::process::exit(0); }

    /* 定义镜像 https 链接 */
    let cargo_https =  r#"
[source.crates-io]
replace-with = 'mirror'

[source.mirror]
registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"

[registries.mirror]
index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
    "#;

    /* 添加镜像 */
    println!("\n镜像内容:\n{}\n",cargo_https);
    let mut file = if let Ok(e) = OpenOptions::new().append(true).create(true).open(cargo_path).await{e}else { println!("$HOME/.cargo/config.toml 创建失败"); std::process::exit(0) };
    let _ = file.write_all(cargo_https.as_bytes()).await?;
    println!("镜像添加成功,可执行 nano $HOME/.cargo/config.toml 命令查看\n若发现重复镜像可放心删除");

    Ok(())
}

/* zigbuild 构建工具 */
pub async fn zigbuild() -> Result<(), Box<dyn std::error::Error>> {
    /* 定义 .cargo 路径 */
    let cargo_path = res_path(".cargo/config.toml");

    /* 镜像存在性判断 */
    if let Ok(mut e) = OpenOptions::new().read(true).open(&cargo_path).await {
        let mut buf = String::new();
        e.read_to_string(&mut buf).await?;

        /* 定义镜像关键部分 */
        let https = vec![
            "[source.crates-io]",
            "replace-with = 'mirror'",
            "[source.mirror]",
            r#"registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/""#,
            "[registries.mirror]",
            r#"index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/""#
        ];

        /* 镜像存在性判断 */
        let mut all = true;
        for i in https { if !buf.contains(i) { all = false; break; }  }
        if all { println!("cargo 镜像已存在,无需添加") }

        /* 安装 zigbuild */
        let mut ins = Command::new("sh")
            .arg("-c")
            .arg(r#"cargo install --locked cargo-zigbuild"#)
            .stdout(Stdio::inherit())/* 输出打印到终端 */
            .stderr(Stdio::inherit())/* 错误打印到终端 */
            .spawn()?;

        /* 堵塞等待安装完成 */
        let res_ins = ins.wait().await?;
        if res_ins.success() { println!("zigbuild安装成功") }else { println!("rust 安装失败") }
    };

    Ok(())
}

/* 更新 rust */
pub async fn update() -> Result<(), Box<dyn std::error::Error>> {
    /* 更新 rust */
    let mut ins = Command::new("sh")
        .arg("-c")
        .arg(r#"rustup update"#)
        .stdout(Stdio::inherit())/* 输出打印到终端 */
        .stderr(Stdio::inherit())/* 错误打印到终端 */
        .spawn()?;

    /* 堵塞等待安装完成 */
    let res_ins = ins.wait().await?;
    if res_ins.success() { println!("rust更新成功") }else { println!("rust更新失败,请安装rust后尝试") }

    Ok(())
}

/* 代码仓库跳转 */
pub async fn br() -> Result<(), Box<dyn std::error::Error>> {
    /* 跳转代码仓库 */
    #[cfg(target_os = "linux")]
    let _ = Command::new("xdg-open")
        .arg("")
        .stdout(Stdio::inherit())/* 输出打印到终端 */
        .stderr(Stdio::inherit())/* 错误打印到终端 */
        .spawn()?;

    Ok(())
}

/* 路径处理 */
fn res_path(path:&str) -> PathBuf {
    /* 获取 $HOME 环境变量并转换为 $Path */
    let home = var("HOME").expect("解包失败");
    let home = Path::new(&home);
    /* join() 不接受 String 所以转化为 &Path */

    /* 路径合并 */
    let path = home.join(path);

    /* 返回合并后的路径 */
    path
}