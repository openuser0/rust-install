//! #类型操作 模块 ( repl_struct.rs )

//! ##功能

//! 封装类型操作

//! ##作者

//! songjiaqicode

//! ##初始化日期

//! 2025.7.10

/* 引入标准库 */

#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::process::Stdio;

#[cfg(any(target_os = "windows", target_os = "macos"))]
use tokio::process::Command;

#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::env::var;

/* 引入私有库 */



/* 内部操作 */

pub async fn run(){
    /* windows 安装逻辑 */
    #[cfg(target_os = "windows")]
    let _ = windows_install().await;

    /* macos 安装逻辑 */
    #[cfg(target_os = "macos")]
    macos_install();
}

/* windows 安装 */
#[cfg(target_os = "windows")]
async fn windows_install() -> Result<(), Box<dyn std::error::Error>>{
    /* 检测 rust 工具是否存在性 */
    if let Ok(_) = Command::new("rustup").arg("--version").output().await { println!("rust 工具已存在"); return Ok(()); }else { println!("未安装rust,现在开始安装"); () }

    /* 获取 USERPROFILE 环境变量 */
    let user = format!("{}\\Desktop\\rustup-init.exe",var("USERPROFILE")?);

    /* 下载 rustup.exe */
    let mut cmd = Command::new("curl.exe")
        .arg("-o")
        .arg(&user)
        .arg("https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe")
        .stdout(Stdio::inherit())/* 输出打印到终端 */
        .stderr(Stdio::inherit())/* 错误打印到终端 */
        .spawn()?;

    /* 堵塞等待下载完成 */
    let _ = cmd.wait().await?;

    /* 执行 rustup.exe  */
    let mut cmd = Command::new(&user).stdout(Stdio::inherit()).stderr(Stdio::inherit()).spawn()?;

    /* 堵塞等待安装完成 */
    let _ = cmd.wait().await?;

    Ok(())
}

/* macos 安装 */
#[cfg(target_os = "macos")]
fn macos_install(){
    println!("我是macos安装程序");
}

