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



/* 代码仓库跳转 */
pub async fn jump() -> Result<(), Box<dyn std::error::Error>> {
    /* 跳转代码仓库 */
    cmd(r#"xdg-open https://gitee.com/songjiaqicode/rust-installation"#,"web启动").await?;
    /* 这会调用 xdg-open(桌面通用web接口) 打开代码仓库 */

    Ok(())
}

/* 添加 cargo 镜像<1.68版本以上> */
pub async fn cargo() -> Result<(), Box<dyn std::error::Error>> {
    /* cargo 镜像存在性检测 */
    let cargo_path = if let Ok(e) = cargo_bool().await{e}else { println!("cargo 镜像已存在,无需添加"); std::process::exit(0) };

    /* 确认rust版本 */
    println!("\n如果你的rust版本低于1.68,或者未安装rust,切勿添加镜像,否则会报错☠️\n\n执行 cargo -V&&rustc -V 命令来确认 rust 版本以及是否安装\n\n你是否已安装 rust 并且版本大于或等于 1.68? [y/n]");
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
    let mut file = if let Ok(e) = OpenOptions::new().append(true).create(true).open(&cargo_path).await{e}else { println!("$HOME/.cargo/config.toml 创建失败"); std::process::exit(0) };
    let _ = file.write_all(cargo_https.as_bytes()).await?;
    println!("镜像添加成功,可执行 nano $HOME/.cargo/config.toml 命令查看\n若发现重复镜像可放心删除");

    Ok(())
}

/* 添加 zigbuild 构建工具 */
pub async fn zigbuild() -> Result<(), Box<dyn std::error::Error>> {
    /* cargo 镜像存在性检测 */
    if let Err(_) = cargo_bool().await{}else { println!("cargo 镜像不存在,请执行 rust-installation cargo 命令添加"); std::process::exit(0) }

    /* 安装 zigbuild */
    cmd(r#"cargo install --locked cargo-zigbuild"#,"zigbuild").await?;

    Ok(())
}

/* 删除 zigbuild 构建工具 */
pub async fn remove_zigbuild() -> Result<(), Box<dyn std::error::Error>> {
    /* 删除 zigbuild */
    cmd(r#"cargo uninstall cargo-zigbuild"#,"zigbuild").await?;

    Ok(())
}

/* 更新 rust */
pub async fn update() -> Result<(), Box<dyn std::error::Error>> {
    /* cargo 镜像存在性检测 */
    if let Err(_) = cargo_bool().await{}else { println!("cargo 镜像不存在,请执行 rust-installation cargo 命令添加"); std::process::exit(0) }

    /* 更新 rust */
    cmd(r#"rustup update"#,"rust").await?;

    Ok(())
}

/* 删除 uninstall */
pub async fn uninstall() -> Result<(), Box<dyn std::error::Error>> {
    /* 删除 uninstall */
    cmd(r#"rustup self uninstall"#,"uninstall").await?;

    Ok(())
}

/* fish 的 tap 补全 */
pub async fn shell_tap() -> Result<(), Box<dyn std::error::Error>> {
    /* 判断 fish 存在性 */
    let res = Command::new("fish").arg("-v").stdout(Stdio::null()).stderr(Stdio::null()).spawn();
    if let Ok(_) = res { println!("fish存在,正在创建配置文件") } else { println!("fish不存在"); std::process::exit(0) };

    /* 定义 fish tap 文件 */
    let file = OpenOptions::new().read(true).write(true).create(true).open(res_path(".config/fish/completions/rust-installation.fish")).await;

    /* 判断创建正确性 */
    let mut file = if let Ok(e) = file { println!("fish tap 文件创建成功"); e }else { println!("fish tap 文件创建失败"); std::process::exit(0) };

    /* 定义写入内容 */
    let write = b"complete -c rust-installation -f -a 'h v c cargo zigbuild remove-zigbuild update uninstall tap list install-nightly remove-nightly install-stable nightly stable'";

    /* 写入文件 */
    let _ = file.write_all(write).await?;

    /* 立刻激活 */
    Command::new("fish").arg("-c").arg(r#"source $HOME/.config/fish/completions/rust-installation.fish"#).spawn()?;

    Ok(())
}

/* 列出所有 rust 版本 */
pub async fn list() -> Result<(), Box<dyn std::error::Error>> {
    /* 判断 rust 存在性 */
    let res = Command::new("rustup").arg("--version").stdout(Stdio::null()).stderr(Stdio::null()).spawn();
    if let Ok(_) = res {}else { println!("rust 不存在 , 执行 rust-installation 命令安装"); std::process::exit(0) };

    /* 列出所有 rust 版本 */
    cmd(r#"rustup show"#,"list").await?;

    Ok(())
}

/* 安装 rust nightly 版本 */
pub async fn install_nightly() -> Result<(), Box<dyn std::error::Error>> {
    cmd(r#"rustup toolchain install nightly"#,"nightly").await?;

    Ok(())
}

/* 删除 rust nightly 版本 */
pub async fn remove_nightly() -> Result<(), Box<dyn std::error::Error>> {
    cmd(r#"rustup toolchain uninstall nightly"#,"remove-nightly").await?;
    Ok(())
}

/* 安装 rust stable 版本 */
pub async fn install_stable() -> Result<(), Box<dyn std::error::Error>> {
    cmd(r#"rustup toolchain install stable"#,"stable").await?;
    Ok(())
}

/* 切换 rust nightly 版本 */
pub async fn nightly() -> Result<(), Box<dyn std::error::Error>> {
    cmd(r#"rustup default nightly"#,"nightly").await?;
    Ok(())
}

/* 切换 rust stable 版本 */
pub async fn stable() -> Result<(), Box<dyn std::error::Error>> {
    cmd(r#"rustup default stable"#,"stable").await?;
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

/* cargo 镜像存在性检测 */
async fn cargo_bool() -> Result<PathBuf, Box<dyn std::error::Error>> {
    /* 定义 .cargo 配置文件路径 */
    let cargo_path = res_path(".cargo/config.toml");

    /* 打开 $HOME/.cargo/config.toml 配置文件 */
    if let Ok(mut e) = OpenOptions::new().read(true).open(&cargo_path).await {

        /* 读取 config.toml 配置文件内容 */
        let mut buf = String::new();
        let _ = e.read_to_string(&mut buf).await;
        /* buf 存放了 config.toml 配置文件内容 */

        /* 定义镜像关键部分 */
        let https = vec![
            "[source.crates-io]",
            "replace-with = 'mirror'",
            "[source.mirror]",
            r#"registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/""#,
            "[registries.mirror]",
            r#"index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/""#
        ];

        /* 代表存在性的变量 */
        let mut all = true;
        /* 默认完整存在 */

        /* 遍历镜像关键部分 */
        for i in https {
            /* 判断 buf 是否完整包含 https 的成员切片  */
            if !buf.contains(i) {
                /* 不完整处理 */
                all = false; break;
                /* 将存在性变量改为 false 并退出遍历 */
            }
        }

        /* 根据存在性变量执行对应的操作 */
        if all { return Err("镜像已存在".into()) }
        /* true 返回值 , false 返回错误 */
    };

    /* 返回 .cargo 配置文件路径 */
    Ok(cargo_path)
}

/* 堵塞命令 */
async fn cmd(shell:&str, pr:&str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ins = Command::new("bash")
        .arg("-c")
        .arg(shell)
        .stdout(Stdio::inherit())/* 输出打印到终端 */
        .stderr(Stdio::inherit())/* 错误打印到终端 */
        .spawn()?;

    /* 堵塞等待安装完成 */
    let res_ins = ins.wait().await?;
    if res_ins.success() { println!("\n{} 成功",pr) }else { println!("{} 失败",pr) }

    Ok(())
}