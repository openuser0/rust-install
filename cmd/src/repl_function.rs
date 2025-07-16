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
            "h" => {
                /* 定义命令参数集合 */
                let cmd = vec![
                    "无参数 , 安装 rustup 并设置镜像",
                    "h , 帮助 ",
                    "v , 版本号",
                    "c , 代码仓库",
                    "cargo , 添加 cargo 镜像",
                    "zigbuild , 添加 zigbuild 构建工具",
                    "remove-zigbuild , 删除 zigbuild 构建工具",
                    "update , 更新 rust",
                    "uninstall , 删除 rust",
                    "tap , 开启 fish 的 tap 补全",
                    "list , 列出所有 rust 版本",
                    "install-nightly , 安装 rust nightly 版本",
                    "remove-nightly , 删除 rust nightly 版本",
                    "install-stable , 安装 rust stable 版本(默认)",
                    "nightly , 切换到 rust nightly 版本",
                    "stable , 切换到 rust stable 版本",
                ];

                /* 打印参数命令信息 */
                for i in cmd { println!("{i}") }

                /* 退出程序 */
                std::process::exit(0)
                /* 防止影响到 func 包中的模块 */
            }

            /* 版本号 */
            "v" => {
                /* 打印版本号 */
                println!("1.2.0");

                /* 退出程序 */
                std::process::exit(0)
                /* 防止影响到 func 包中的模块 */
            }

            /* 代码仓库 */
            "c" => {
                /* 通过 web 接口跳转代码仓库 */
                if let Ok(_) = function_mod_https::jump().await {}else { println!("代码仓库跳转失败"); }

                /* 打印代码仓库 */
                println!("gitcode:\nhttps://gitcode.com/songjiaqicode/rust-installation\ngitee:\nhttps://gitee.com/songjiaqicode/rust-installation");

                /* 退出程序 */
                std::process::exit(0)
                /* 防止影响到 func 包中的模块 */
            }

            /* 1.68 以上版本 cargo 镜像 */
            "cargo" => {
                /* 抽象化选择 */
                select("是否添加 cargo 镜像? [y/n]", "已取消 cargo 镜像添加");
                function_mod_https::cargo().await?;

                /* 退出程序 */
                std::process::exit(0)
                /* 防止影响到 func 包中的模块 */
            }

            /* zigbuild 构建工具 */
            "zigbuild" => {
                /* 抽象化选择 */
                select("是否安装 cargo-zigbuild 构建工具? [y/n]", "已取消安装 zigbuild");
                function_mod_https::zigbuild().await?;

                /* 退出程序 */
                std::process::exit(0)
                /* 防止影响到 func 包中的模块 */
            }

            /* 删除 zigbuild 构建工具 */
            "remove-zigbuild" => {
                /* 抽象化选择 */
                select("是否删除 cargo-zigbuild 构建工具? [y/n]", "已取消删除 zigbuild");
                function_mod_https::remove_zigbuild().await?;

                /* 退出程序 */
                std::process::exit(0)
                /* 防止影响到 func 包中的模块 */
            }

            /* 更新 rust */
            "update" => {
                /* 抽象化选择 */
                select("是否更新 rust? [y/n]", "已取消 rust 更新");
                function_mod_https::update().await?;

                /* 退出程序 */
                std::process::exit(0)
                /* 防止影响到 func 包中的模块 */
            }

            /* 删除 rust */
            "uninstall" => {
                /* 抽象化选择 */
                select("是否删除 rust? [y/n]", "已取消 rust 删除");
                function_mod_https::uninstall().await?;

                /* 退出程序 */
                std::process::exit(0)
                /* 防止影响到 func 包中的模块 */
            }

            /* 开启 fish 的 tap 补全 */
            "tap" => {
                let _ = function_mod_https::shell_tap().await;

                /* 退出程序 */
                std::process::exit(0)
                /* 防止影响到 func 包中的模块 */
            }

            /* 列出所有 rust 版本 */
            "list" => {
                let _ = function_mod_https::list().await;

                /* 退出程序 */
                std::process::exit(0)
                /* 防止影响到 func 包中的模块 */
            }

            /* 安装 rust nightly 版本 */
            "install-nightly" => {
                /* 抽象化选择 */
                select("是否安装 rust nightly ？ [y/n]", "已取消 nightly 添加");
                let _ = function_mod_https::install_nightly().await;

                /* 退出程序 */
                std::process::exit(0)
                /* 防止影响到 func 包中的模块 */
            }

            /* 删除 rust nightly 版本 */
            "remove-nightly" => {
                /* 抽象化选择 */
                select("是否删除 rust nightly ？ [y/n]", "已取消 nightly 删除");
                let _ = function_mod_https::remove_nightly().await;

                /* 退出程序 */
                std::process::exit(0)
                /* 防止影响到 func 包中的模块 */
            }

            /* 安装 rust stable 版本 */
            "install-stable" => {
                /* 抽象化选择 */
                select("是否安装 rust stable ？ [y/n]", "已取消 stable 添加");
                let _ = function_mod_https::install_stable().await;

                /* 退出程序 */
                std::process::exit(0)
                /* 防止影响到 func 包中的模块 */
            }

            /* 切换 rust nightly 版本 */
            "nightly" => {
                select("是否切换到 rust nightly ？ [y/n]", "已取消 nightly 切换");
                let _ = function_mod_https::nightly().await;

                /* 退出程序 */
                std::process::exit(0)
                /* 防止影响到 func 包中的模块 */
            }

            /* 切换 rust stable 版本 */
            "stable" => {
                select("是否切换到 rust stable ？ [y/n]", "已取消 stable 切换");
                let _ = function_mod_https::stable().await;

                /* 退出程序 */
                std::process::exit(0)
                /* 防止影响到 func 包中的模块 */
            }

            /* 错误命令处理 */
            _ => { println!("未定义的命令"); std::process::exit(0);}
        }
    }

    Ok(())
}

/* 通用选择 */
fn select(pr:&str, rn:&str) {
    /* 安装选择 */
    println!("{}",pr);
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);

    /* 判断选择 */
    if buf.trim() == "y" { () }else { println!("{}",rn); std::process::exit(0); }
}