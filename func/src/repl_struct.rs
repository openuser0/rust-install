//! #类型操作 模块 ( repl_struct.rs )

//! ##功能

//! 封装类型操作

//! ##作者

//! songjiaqicode

//! ##初始化日期

//! 2025.7.10

/* 引入标准库 */

#[cfg(any(target_os = "windows", target_os = "macos"))]
use tokio::process::Command;

/* 引入私有库 */



/* 内部操作 */

pub async fn run(){
    /* windows 安装逻辑 */
    #[cfg(target_os = "windows")]
    if let Ok(_) = windows_install().await {}else { println!("windows 安装失败") }; std::process::exit(0);

    /* macos 安装逻辑 */
    #[cfg(target_os = "macos")]
    macos_install();
}

/* windows 安装 */
#[cfg(target_os = "windows")]
async fn windows_install() -> Result<(), Box<dyn std::error::Error>> {
    /* 检测 rust 工具是否存在性 */
    if let Ok(_) = Command::new("cargo").arg("-V").status().await { println!("rust 已存在"); return Ok(()) }else { println!("rust 不存在 , 开始安装") }

    /* 执行并判断 msys2 bash 存在性 */
    if let Ok(_) = Command::new(r#"C:\msys64\usr\bin\bash.exe"#).args(["-c","bash --version"]).status().await {
        /* 安装 rustup */
        Command::new(r#"C:\msys64\usr\bin\bash.exe"#).args(["-c","curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"])
            /* 设置管道 | 临时镜像 */
            .env("RUSTUP_UPDATE_ROOT","https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup")
            .env("RUSTUP_DIST_SERVER","https://mirrors.tuna.tsinghua.edu.cn/rustup")
            .status().await?;
    }else {
        /* 安装 msys2 bash */
        println!("msys2 bash 不存在 , 尝试安装");
        if let Ok(_) = Command::new(r#".\msys2.exe"#).status().await {}else {
            Command::new("curl").args(["-sSLo","msys2.exe","https://mirrors.tuna.tsinghua.edu.cn/msys2/distrib/x86_64/msys2-x86_64-20250622.exe"]).status().await?;
            Command::new(r#".\msys2.exe"#).status().await?;
        }

        /* 安装 rustup */
        Command::new(r#"C:\msys64\usr\bin\bash.exe"#).args(["-c","curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"])
            /* 设置管道 | 临时镜像 */
            .env("RUSTUP_UPDATE_ROOT","https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup")
            .env("RUSTUP_DIST_SERVER","https://mirrors.tuna.tsinghua.edu.cn/rustup")
            .status().await?;
    }

    Ok(())
}

/* macos 安装 */
#[cfg(target_os = "macos")]
fn macos_install(){
    println!("我是macos安装程序");
}

