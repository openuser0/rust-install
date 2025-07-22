//! #函数操作 模块 ( function_mod.rs )

//! ##功能

//! 封装函数操作

//! ##作者

//! songjiaqicode

//! ##初始化日期

//! 2025.7.11

/* 引入标准库 */

use std::io::stdin;
use std::path::{PathBuf, Path};
use std::env::var;
use tokio::fs::{OpenOptions};
use tokio::io::{AsyncWriteExt};
use tokio::process::Command;

/* 引入私有库 */

/* 选择枚举 */
pub enum Select{
    H,/* 帮助 */
    V,/* 版本号 */
    C,/* 代码仓库 */
    List,/* 列出rust版本信息 */

    Cargo,/* 添加cargo镜像 */
    TapFish,/* 开启fish shell tap补全 */
    TapBash,/* 开启bash shell tap补全 */
    InstallNightly,/* 安装rust-nightly(每日构建)版本 */
    Nightly,/* 切换到rust-nightly版本 */
    RemoveNightly,/* 删除rust-nightly版本 */
    Stable,/* 切换到rust-stable版本 */
    Uninstall,/* 删除rust */
    Update,/* 更新rust */

    Zigbuild,/* 添加zigbuild构建工具 */
    DocZigbuild,/* 文档 */
    RemoveZigbuild,/* 删除zigbuild */

    Tauri,/* 添加tauri框架 */
    DocTauri,/* 文档 */
    RemoveTauri,/* 删除tauri */
}

/* 根据枚举值匹配操作 */
pub async fn select(par:Select){
    /* 匹配选择枚举值 */
    match par {
        Select::H =>{ help() }

        Select::V =>{ println!("1.5.0"); std::process::exit(0) }

        Select::C => { if let Ok(_) = jump("gitcode.com/songjiaqicode/rust-installation").await {}else { err() } off() }

        Select::List => { select_cmd("是否列出 rust 版本信息? [y/n]"); if let Ok(_) = rustup_cli("show").await {}else { err() } off() }

        Select::Cargo => { select_cmd("是否添加 cargo 镜像? [y/n]"); if let Ok(_) = cargo().await {}else { err() } off() }

        Select::TapFish => { select_cmd("是否开启fish shell tap补全? [y/n]"); if let Ok(_) = tap_fish().await {}else { err() } off() }

        Select::TapBash => { select_cmd("是否开启bash shell tap补全? [y/n]"); if let Ok(_) = tap_bash().await {}else { err() } off() }

        Select::InstallNightly => { select_cmd("是否安装 rust nightly 版本? [y/n]"); if let Ok(_) = rustup_cli("install nightly").await {}else { err() } off() }

        Select::Nightly => { select_cmd("是否切换到 nightly 版本? [y/n]"); if let Ok(_) = rustup_cli("default nightly").await {} else { err() } off() }

        Select::RemoveNightly => { select_cmd("是否删除 rust nightly 版本? [y/n]"); if let Ok(_) = rustup_cli("toolchain uninstall nightly").await {} else { err() } off() }

        Select::Stable => { select_cmd("是否切换到 stable 版本? [y/n]"); if let Ok(_) = rustup_cli("default stable").await {} else { err() } off() }

        Select::Uninstall => { select_cmd("是否删除 rust? [y/n]"); if let Ok(_) = rustup_cli("self uninstall").await {}else { err() } off() }

        Select::Update => { select_cmd("是否更新 rust? [y/n]"); if let Ok(_) = rustup_cli("update").await {}else { err() } off() }

        Select::Zigbuild => { select_cmd("是否添加 cargo-zigbuild 构建工具? [y/n]"); if let Ok(_) = cargo_cli("install --locked cargo-zigbuild").await {}else { err() } off() }

        Select::DocZigbuild => { select_cmd("是否查看 cargo-zigbuild 文档? [y/n]"); if let Ok(_) = jump("https://juejin.cn/post/7527206638262599706").await {}else { err() } off() }

        Select::RemoveZigbuild => { select_cmd("是否删除 cargo-zigbuild 构建工具? [y/n]"); if let Ok(_) = cargo_cli("uninstall cargo-zigbuild").await {}else { err() } off() }

        Select::Tauri => { select_cmd("是否添加 tauri 框架? [y/n]"); if let Ok(_) = cargo_cli("install create-tauri-app --locked").await {}else { err() } off() }

        Select::DocTauri => { select_cmd("是否查看 tauri 文档? [y/n]"); if let Ok(_) = jump("https://v2.tauri.org.cn/start/").await {}else { err() } off() }

        Select::RemoveTauri => { select_cmd("是否删除 tauri 框架? [y/n]"); if let Ok(_) = cargo_cli("uninstall create-tauri-app").await {} else { err() } off() }
    }
}

/* 帮助 */
pub fn help(){
    let cmd = vec![
        "无参数 安装rust(自动配置镜像)\n",
        "[非功能性]",
        "h      帮助",
        "v      版本号",
        "c      代码仓库",
        "list   列出rust版本信息\n",
        "[辅助性]",
        "cargo              添加cargo镜像",
        "tap-fish           开启fish shell tap补全",
        "tap-bash           开启bash shell tap补全",
        "install-nightly    安装rust-nightly(每日构建)版本",
        "nightly            切换到rust-nightly版本",
        "remove-nightly     删除rust-nightly版本",
        "stable             切换到rust-stable版本",
        "uninstall          删除rust",
        "update             更新rust\n",
        "[zigbuild构建工具]",
        "zigbuild           添加zigbuild构建工具",
        "doc-zigbuild       文档",
        "remove-zigbuild    删除zigbuild\n",
        "[tauri前端框架]",
        "tauri          添加tauri框架",
        "doc-tauri      文档",
        "remove-tauri   删除tauri\n",
    ];
    for i in cmd { println!("{i}") } std::process::exit(0)
}

/* 通用跳转 */
pub async fn jump(https:&str) -> Result<(), Box<dyn std::error::Error>> {
    /* 跳转代码仓库 */
    #[cfg(target_os = "linux")]
    cmd(&format!("xdg-open {}",https)).await?;

    #[cfg(target_os = "windows")]
    Command::new("cmd").args(["/C","start",https]).status().await?;

    Ok(())
}

/* 开启fish shell tap补全 */
pub async fn tap_fish() -> Result<(), Box<dyn std::error::Error>> {
    /* 判断 fish 存在性 */
    let res = Command::new("fish").arg("-v").status().await;
    if let Ok(_) = res { println!("fish存在,正在创建配置文件") } else { println!("fish不存在"); return Err("fish不存在".into()) };

    /* 创建 fish tap 补全并写入内容 */
    let file = OpenOptions::new().read(true).write(true).create(true).open(res_path(".config/fish/completions/rust-install.fish")).await;
    let mut file = if let Ok(e) = file { println!("fish tap 文件创建成功"); e }else { println!("fish tap 文件创建失败"); return Err("fish不存在".into()) };
    let write = b"complete -c rust-install -f -a 'h v c list cargo tap-fish tap-bash install-nightly nightly remove-nightly stable uninstall update zigbuild doc-zigbuild remove-zigbuild tauri doc-tauri remove-tauri'";
    let _ = file.write_all(write).await?; println!("重启终端后即可使用 fish tap补全");

    Ok(())
}

/* 开启bash shell tap补全 */
pub async fn tap_bash() -> Result<(), Box<dyn std::error::Error>>{
    /* 判断 fish 存在性 */
    let res = Command::new("bash").arg("--version").status().await;
    if let Ok(_) = res { println!("bash存在,正在创建配置文件") } else { println!("bash不存在"); return Err("bash不存在".into()) };

    let file = OpenOptions::new().append(true).read(true).create(true).open(res_path(".bashrc")).await;
    let mut file = if let Ok(e) = file { println!("bash tap 文件创建成功"); e }else { println!("bash tap 文件创建失败"); return Err("bash tap 文件创建失败".into()) };
    let write = r#"complete -W "h v c list cargo tap-fish tap-bash install-nightly nightly remove-nightly stable uninstall update zigbuild doc-zigbuild remove-zigbuild tauri doc-tauri remove-tauri" rust-install"#;
    let _ = file.write_all(write.as_bytes()).await?; println!("重启终端后即可使用 bash tap补全");

    Ok(())
}

/* 添加 cargo 镜像<1.68版本以上> */
pub async fn cargo() -> Result<(), Box<dyn std::error::Error>> {
    /* 添加镜像 */
    select_cmd("警告⚠️:重复添加镜像会覆盖现有配置,很可能破坏开发环境\n执行 nano $HOME/.cargo/config.toml 命令查看镜像是否存在\n是否添加 cargo 镜像? [y/n]");
    cmd(r#"mkdir -vp ${CARGO_HOME:-$HOME/.cargo}

cat << EOF | tee -a ${CARGO_HOME:-$HOME/.cargo}/config.toml
[source.crates-io]
replace-with = 'mirror'

[source.mirror]
registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"

[registries.mirror]
index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
EOF"#).await?;

    Ok(())
}

/* 通用 rustup 命令 */
async fn rustup_cli(shell:&str) -> Result<(), Box<dyn std::error::Error>> { cmd(&format!("rustup {}",shell)).await?; Ok(()) }

/* 通用 cargo 命令 */
async fn cargo_cli(shell:&str)-> Result<(), Box<dyn std::error::Error>> { cmd(&format!("cargo {}",shell)).await?; Ok(()) }

/* 路径处理 */
fn res_path(path:&str) -> PathBuf {
    /* 合并 $HOME 环境变量和参数 */
    let home = var("HOME").expect("$HOME 环境变量不存在");
    let home = Path::new(&home);
    let path = home.join(path);

    path
}

/* 执行命令 */
async fn cmd(shell:&str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    Command::new("bash").arg("-c").arg(shell)
        /* 设置管道 | 临时镜像 */
        .env("RUSTUP_UPDATE_ROOT","https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup")
        .env("RUSTUP_DIST_SERVER","https://mirrors.tuna.tsinghua.edu.cn/rustup")
        .status().await?;

    #[cfg(target_os = "windows")]
    Command::new(r#"C:\msys64\usr\bin\bash.exe"#).args(["-c",shell])
        .env("RUSTUP_UPDATE_ROOT","https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup")
        .env("RUSTUP_DIST_SERVER","https://mirrors.tuna.tsinghua.edu.cn/rustup")
        .status().await?;

    Ok(())
}

/* 通用选择 */
fn select_cmd(pr:&str) {
    /* 选择 y/n  */
    println!("{}",pr);
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    if buf.trim() == "y" { () }else { println!("操作已取消"); std::process::exit(0); }
}

/* 通用关闭 */
fn off(){ std::process::exit(0) }

/* 通用报错 */
fn err(){ println!("错误☠️"); std::process::exit(0) }