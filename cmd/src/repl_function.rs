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

/* 参数命令处理模块 */
#[path = "function_mod.rs"]
mod function_mod;

use function_mod::Select;

/* 内部操作 */

pub async fn run() -> Result<(), Box<dyn std::error::Error>>{
    /*
    目标: 获取并解析命令参数,执行对应的操作
    支持平台: linux
    */

    /* 获取命令行参数 */
    for i in args().skip(1) {
        match i.as_str() {

            "h" => { function_mod::select(Select::H).await }

            "v" => { function_mod::select(Select::V).await }

            "c" => { function_mod::select(Select::C).await }

            "list" => { function_mod::select(Select::List).await }

            "cargo" => { function_mod::select(Select::Cargo).await }

            "tap-fish" => { function_mod::select(Select::TapFish).await }

            "tap-bash" => { function_mod::select(Select::TapBash).await }

            "install-nightly" => { function_mod::select(Select::InstallNightly).await }

            "nightly" => { function_mod::select(Select::Nightly).await }

            "remove-nightly" => { function_mod::select(Select::RemoveNightly).await }

            "stable" => { function_mod::select(Select::Stable).await}

            "uninstall" => { function_mod::select(Select::Uninstall).await }

            "update" => { function_mod::select(Select::Update).await }

            "zigbuild" => { function_mod::select(Select::Zigbuild).await }

            "doc-zigbuild" => { function_mod::select(Select::DocZigbuild).await }

            "remove-zigbuild" => { function_mod::select(Select::RemoveZigbuild).await }

            "tauri" => { function_mod::select(Select::Tauri).await }

            "doc-tauri" => { function_mod::select(Select::DocTauri).await }

            "remove-tauri" => { function_mod::select(Select::RemoveTauri).await }

            _ => { println!("未定义的命令"); std::process::exit(0);}
        }
    }

    Ok(())
}