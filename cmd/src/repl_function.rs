//! #函数操作 模块 ( repl_function.rs )

//! ##功能

//! 封装函数操作

//! ##作者

//! songjiaqicode

//! ##初始化日期

//! 2025.7.11

/* 引入标准库 */

use std::env::{args};

/* 引入私有库 */

#[path = "function_mod_https.rs"]
mod function_mod_https;

use function_mod_https::Select;

/* 内部操作 */

pub async fn run() -> Result<(), Box<dyn std::error::Error>>{
    /*
    目标: 获取并解析命令参数,执行对应的操作
    支持平台: linux
    */

    /* 获取命令行参数 */
    for i in args().skip(1) {
        /* i 存放的是 String */

        match i.as_str() {
            /* as_str() 将 String 转化为 &str */

            /* 帮助 */
            "h" => { function_mod_https::select(Select::H).await }

            /* 版本号 */
            "v" => { function_mod_https::select(Select::V).await }

            /* 代码仓库 */
            "c" => { function_mod_https::select(Select::C).await }

            /* 1.68 以上版本 cargo 镜像 */
            "cargo" => { function_mod_https::select(Select::Cargo).await }

            /* 安装 rust nightly 版本 */
            "install-nightly" => { function_mod_https::select(Select::InstallNightly).await }

            /* 安装 rust stable 版本 */
            "install-stable" => { function_mod_https::select(Select::InstallStable).await}

            /* 列出所有 rust 版本 */
            "list" => { function_mod_https::select(Select::List).await }

            /* 切换 rust nightly 版本 */
            "nightly" => { function_mod_https::select(Select::Nightly).await }

            /* 删除 rust nightly 版本 */
            "remove-nightly" => { function_mod_https::select(Select::RemoveNightly).await }

            /* 删除 zigbuild 构建工具 */
            "remove-zigbuild" => { function_mod_https::select(Select::RemoveZigbuild).await }

            /* 切换 rust stable 版本 */
            "stable" => { function_mod_https::select(Select::Stable).await}

            /* 开启 fish 的 tap 补全 */
            #[cfg(target_os = "linux")]
            "tap" => { function_mod_https::select(Select::Tap).await }

            /* 删除 rust */
            "uninstall" => { function_mod_https::select(Select::Uninstall).await }

            /* 更新 rust */
            "update" => { function_mod_https::select(Select::Update).await }

            /* zigbuild 构建工具 */
            "zigbuild" => { function_mod_https::select(Select::Zigbuild).await }

            /* 错误命令处理 */
            _ => { println!("未定义的命令"); std::process::exit(0);}
        }
    }

    Ok(())
}