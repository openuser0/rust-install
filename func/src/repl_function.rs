//! #函数操作 模块 ( repl_function.rs )

//! ##功能

//! 封装函数操作

//! ##作者

//! songjiaqicode

//! ##初始化日期

//! 2025.7.10

/* 引入标准库 */

use std::path::PathBuf;
use std::env::{var};
use std::path::Path;
use std::process::{ Stdio };
use tokio::fs::{OpenOptions};
use tokio::io::{AsyncWriteExt};
use tokio::process::Command;

/* 引入私有库 */



/* 内部操作 */

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /*
    目标: 实现自动化安装rust以及设置镜像
    支持平台: linux
    */

    /* 检测 rust 工具是否存在 */
    if let Ok(_) = Command::new("rustup").arg("--version").output().await { println!("rust 工具已存在"); return Ok(()); }else { println!("未安装rust,现在开始安装"); () }

    /* 定义 shell 配置文件路径 */
    let bash = res_path(".bashrc");
    let fish = res_path(".config/fish/config.fish");
    /* 合并 $HOME 路径和 bash fish 的配置文件路径为 PathBuf */

    /* 定义 shell 镜像 */
    let bash_https = "export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup\nexport RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup";
    let fish_https = "set -x RUSTUP_UPDATE_ROOT https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup\nset -x RUSTUP_DIST_SERVER https://mirrors.tuna.tsinghua.edu.cn/rustup";

    /* 根据 shell 添加镜像 */
    if let Ok(_) = Command::new("bash").arg("--version").output().await { shell_https("bash",bash,bash_https).await? }else { println!("bash 不存在") };
    if let Ok(_) = Command::new("fish").arg("--version").output().await { shell_https("fish",fish,fish_https).await? }else { println!("fish 不存在") }


    /* 安装 rustup */
    let mut cmd = Command::new("bash")
        .arg("-c")
        .arg(r#"curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"#)
        .stdout(Stdio::inherit())/* 输出打印到终端 */
        .stderr(Stdio::inherit())/* 错误打印到终端 */
        .spawn()?;

    /* 堵塞等待安装完成 */
    let cmd = cmd.wait().await?;
    if cmd.success() { println!("rust安装成功") }else { println!("rust 安装失败") }

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

/* shell 镜像配置 */
async fn shell_https(name:&str, path:PathBuf, https:&str) -> Result<(), Box<dyn std::error::Error>>{
    if let Ok(_) = Command::new(name).arg("--version").output().await {
        /* 增量写入打开配置文件 */
        let mut file = if let Ok(e) = OpenOptions::new().append(true).create(true).open(&path).await {e}else { println!("配置文件打开失败"); return Err("配置文件打开失败".into()) };

        /* 写入镜像配置 */
        file.write_all(https.as_bytes()).await?;

        /* 让镜像立刻生效 */
        let path = format!(r#"source {}"#, path.display());
        if let Ok(_) = Command::new("bash").arg("-c").arg(&path).output().await { println!("{}镜像已生效",name); }else { println!("镜像生效失败") }
    };

    Ok(())
}