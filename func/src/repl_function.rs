//! #函数操作 模块 ( repl_function.rs )

//! ##功能

//! 封装函数操作

//! ##作者

//! songjiaqicode

//! ##初始化日期

//! 2025.7.10

/* 引入标准库 */

use tokio::process::Command;

/* 引入私有库 */



/* 内部操作 */

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /*
    目标: 实现自动化安装rust以及设置镜像
    支持平台: linux
    */

    /* 检测 rust 工具是否存在性 */
    if let Ok(_) = Command::new("rustup").arg(&format!("-V&&rustc -V&&cargo -V")).status().await { println!("rust 已存在"); return Ok(()) }else { println!("rust 不存在 , 开始安装") }

    /* 临时设置 rustup 镜像(bash fish) */
    if let Ok(_) = Command::new("bash").arg("--version").status().await{
        Command::new("bash").arg(&format!("-c {}",r#"export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup&&export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup"#))
    }else { println!("bash 不存在"); return Err("bash 不存在".into()) };

    /* 安装 rustup */
    Command::new("bash").arg(&format!("-c {}",r#"curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"#)).status().await?;
    
    Ok(())
}