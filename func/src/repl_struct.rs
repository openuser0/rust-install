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
    println!("狗屎windows,微软操你妈");

    Ok(())
}

/* macos 安装 */
#[cfg(target_os = "macos")]
fn macos_install(){
    println!("我是macos安装程序");
}

