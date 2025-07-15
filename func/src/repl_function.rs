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
use tokio::io::{AsyncWriteExt, AsyncReadExt};
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

    /* 定义 bash || fish 镜像关键部分 */
    let bash_file = vec!["export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup","export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup"];
    let fish_file = vec!["set -x RUSTUP_UPDATE_ROOT https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup","set -x RUSTUP_DIST_SERVER https://mirrors.tuna.tsinghua.edu.cn/rustup"];

    /* rustup 镜像存在性检测(bash) */
    if let Err(_) = rustup_bool(".bash_profile",bash_file).await {
        /* 定义 bash 配置文件路径 */
        let bash = res_path(".bash_profile");

        /* 定义 bash rustup 镜像 */
        let bash_https = "export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup\nexport RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup";

        /* 添加 bash rustup 镜像 */
        if let Ok(_) = Command::new("bash").arg("--version").output().await { shell_https("bash",bash,bash_https).await? }else { println!("bash 不存在") };

        /* 立刻激活镜像 */
        if let Ok(_) = Command::new("bash").arg("-c").arg(r#"source $HOME/.bash_profile"#).output().await {}else { println!("bash rustup 镜像激活失败"); }
    }else { println!("bash rustup 镜像已存在") }

    /* rustup 镜像存在性检测(fish) */
    if let Err(_) = rustup_bool(".config/fish/config.fish",fish_file).await {
        /* 定义 fish 配置文件路径 */
        let fish = res_path(".config/fish/config.fish");

        /* 定义 fish rustup 镜像 */
        let fish_https = "set -x RUSTUP_UPDATE_ROOT https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup\nset -x RUSTUP_DIST_SERVER https://mirrors.tuna.tsinghua.edu.cn/rustup";

        /* 根据 fish rustup 添加镜像 */
        if let Ok(_) = Command::new("fish").arg("--version").output().await { shell_https("fish",fish,fish_https).await? }else { println!("fish 不存在") }

        /* 立刻激活镜像 */
        if let Ok(_) = Command::new("fish").arg("-c").arg(r#"source $HOME/.config/fish/config.fish"#).output().await {}else { println!("fish rustup 镜像激活失败"); }
    }else { println!("fish rustup 镜像已存在") }

    /* 安装 rustup */
    let mut cmd = Command::new("bash").arg("-c").arg(r#"curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"#)
        .stdout(Stdio::inherit())/* 输出打印到终端 */
        .stderr(Stdio::inherit())/* 错误打印到终端 */
        .spawn()?;

    /* 堵塞等待安装完成 */
    let cmd = cmd.wait().await?;
    if cmd.success() { println!("rust 安装成功 , 重启终端后才能使用 rustup rustc cargo 以及其他 rust 工具") }else { println!("rust 安装失败") }

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
        if let Ok(_) = Command::new(name).arg("-c").arg(&path).output().await { println!("{}镜像已生效",name); }else { println!("镜像生效失败") }
    };

    Ok(())
}

/* rustup 镜像存在性检测 */
async fn rustup_bool(path:&str, col:Vec<&str>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    /* 定义 shell rustup 配置文件路径 */
    let cargo_path = res_path(path);

    /* 打开 shell rustup 配置文件 */
    if let Ok(mut e) = OpenOptions::new().read(true).create(true).open(&cargo_path).await {

        /* 读取 shell rustup 配置文件内容 */
        let mut buf = String::new();
        let _ = e.read_to_string(&mut buf).await;
        /* buf 存放了 config.toml 配置文件内容 */

        /* 代表存在性的变量 */
        let mut all = true;
        /* 默认完整存在 */

        /* 遍历镜像关键部分 */
        for i in col {
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

    /* 返回 shell rustup 配置文件路径 */
    Ok(cargo_path)
}