//! #函数操作 模块 ( repl_function.rs )

//! ##功能

//! 封装函数操作

//! ##作者

//! songjiaqicode

//! ##初始化日期

//! 2025.7.11

/* 引入标准库 */

use std::env::{args};
use std::io::{stdin};

/* 引入私有库 */

#[path = "function_mod_https.rs"]
mod function_mod_https;

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
            "H" | "h" => {
                println!("h | H , 帮助 ");
                println!("v | V , 版本号");
                println!("c | C , 代码仓库");
                println!("cargo , 添加cargo镜像");
                println!("zigbuild , 添加 zigbuild 构建工具");
                println!("update , 更新 rust");

                /* 退出程序 */
                std::process::exit(0)
            }

            /* 版本号 */
            "V" | "v" => {
                println!("0.1.0");

                /* 退出程序 */
                std::process::exit(0)
            }

            /* 代码仓库 */
            "C" | "c" => {
                function_mod_https::br().await?;

                /* 退出程序 */
                std::process::exit(0)
            }

            /* 1.68 以上版本 cargo 镜像 */
            "cargo" => {
                /* 添加镜像选择 */
                println!("是否添加 cargo 镜像? [y/n]");
                let mut buf = String::new();
                let _ = stdin().read_line(&mut buf);

                /* 判断选择 */
                if buf.trim() == "y" { function_mod_https::cargo().await? }else { println!("已取消镜像添加"); std::process::exit(0); }

                /* 退出程序 */
                std::process::exit(0)
            }

            /* zigbuild 构建工具 */
            "zigbuild" => {
                /* 安装选择 */
                println!("是否安装 cargo-zigbuild 构建工具? [y/n]");
                let mut buf = String::new();
                let _ = stdin().read_line(&mut buf);

                /* 判断选择 */
                if buf.trim() == "y" { function_mod_https::zigbuild().await? }else { println!("已取消安装"); std::process::exit(0); }

                /* 退出程序 */
                std::process::exit(0)
            }

            "update" => {
                /* 更新选择 */
                println!("是否更新 rust? [y/n]");
                let mut buf = String::new();
                let _ = stdin().read_line(&mut buf);

                /* 判断选择 */
                if buf.trim() == "y" { function_mod_https::update().await? }else { println!("已取消安装"); std::process::exit(0); }

                /* 退出程序 */
                std::process::exit(0)
            }

            /* 错误命令处理 */
            _ => { println!("未定义的命令"); std::process::exit(0);}
        }
    }

    Ok(())
}