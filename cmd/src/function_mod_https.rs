//! #函数操作 模块 ( function_mod_https.rs )

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

#[cfg(target_os = "linux")]
use std::process::Stdio;

use tokio::fs::{OpenOptions};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::process::Command;

/* 引入私有库 */

/* 选择枚举 */
pub enum Select{
    H,/* 帮助 */
    V,/* 版本 */
    C,/* 代码仓库 */
    Cargo,/* 添加 cargo 镜像 */
    InstallNightly,/* 安装 rust nightly 版本 */
    InstallStable,/* 安装 rust stable 版本 */
    List,/* 列出所有 rust 版本 */
    Nightly,/* 切换 nightly 版本 */
    RemoveNightly,/* 删除 rust nightly 版本 */
    RemoveZigbuild,/* 删除 zigbuild 构建工具 */
    Stable,/* 切换 stable 版本 */

    #[cfg(target_os = "linux")]
    Tap,/* 创建 fish tap 补全 */

    Uninstall,/* 删除 rust */
    Update,/* 更新 rust */
    Zigbuild,/* 添加 zigbuild 构建工具 */
}

/* 根据传入枚举值进行对应的操作 */
pub async fn select(par:Select){
    /* 匹配枚举值 */
    match par {
        /* 帮助 */
        Select::H =>{ help() }

        /* 版本 */
        Select::V =>{ println!("1.4.0"); std::process::exit(0) }

        /* 代码仓库 */
        Select::C => { if let Ok(_) = jump().await {}else { println!("代码仓库跳转失败")}}

        /* 添加 cargo 镜像 */
        Select::Cargo => {
            /* 抽象化选择 */
            select_cmd("是否添加 cargo 镜像? [y/n]", "已取消 cargo 镜像添加");
            if let Ok(_) = cargo().await {}else { println!("cargo 镜像添加失败")}; std::process::exit(0)
        }

        /* 安装 rust nightly 版本 */
        Select::InstallNightly => {
            select_cmd("是否安装 rust nightly 版本? [y/n]", "已取消安装 rust nightly 版本");
            if let Ok(_) = install_nightly().await {}else { println!("安装 rust nightly 版本失败")}; std::process::exit(0)
        }

        /* 安装 rust stable 版本 */
        Select::InstallStable => {
            select_cmd("是否安装 rust stable 版本? [y/n]", "已取消安装 rust stable 版本");
            if let Ok(_) = install_stable().await {} else { println!("安装 rust stable 版本失败")}; std::process::exit(0)
        }

        /* 列出所有 rust 版本 */
        Select::List => {
            select_cmd("是否列出所有 rust 版本? [y/n]", "已取消列出所有 rust 版本");
            if let Ok(_) = list().await {}else { println!("列出所有 rust 版本失败")}; std::process::exit(0)
        }

        /* 切换到 nightly 版本 */
        Select::Nightly => {
            select_cmd("是否切换到 nightly 版本? [y/n]", "已取消切换到 nightly 版本");
            if let Ok(_) = nightly().await {} else { println!("切换到 nightly 版本失败")}; std::process::exit(0)
        }

        /* 删除 rust nightly 版本 */
        Select::RemoveNightly => {
            select_cmd("是否删除 rust nightly 版本? [y/n]", "已取消删除 rust nightly 版本");
            if let Ok(_) = remove_nightly().await {} else { println!("删除 rust nightly 版本失败")}; std::process::exit(0)
        }

        /* 删除 zigbuild 构建工具 */
        Select::RemoveZigbuild => {
            select_cmd("是否删除 cargo-zigbuild 构建工具? [y/n]", "已取消删除 zigbuild");
            if let Ok(_) = remove_zigbuild().await {}else { println!("zigbuild 删除失败")}; std::process::exit(0)
        }

        /* 切换到 stable 版本 */
        Select::Stable => {
            select_cmd("是否切换到 stable 版本? [y/n]", "已取消切换到 stable 版本");
            if let Ok(_) = stable().await {} else { println!("切换到 stable 版本失败")}; std::process::exit(0)
        }

        /* 创建 fish tap 补全 */
        #[cfg(target_os = "linux")]
        Select::Tap => {
            select_cmd("是否创建 fish tap 补全? [y/n]", "已取消 fish tap 补全创建");
            if let Ok(_) = tap().await {}else { println!("fish tap 补全创建失败")}; std::process::exit(0)
        }

        /* 删除 rust */
        Select::Uninstall => {
            select_cmd("是否删除 rust? [y/n]", "已取消 rust 删除");
            if let Ok(_) = uninstall().await {}else { println!("rust 删除失败")}; std::process::exit(0)
        }

        /* 更新 rust */
        Select::Update => {
            select_cmd("是否更新 rust? [y/n]", "已取消 rust 更新");
            if let Ok(_) = update().await {}else { println!("rust 更新失败")}; std::process::exit(0)
        }

        /* 添加 zigbuild 构建工具 */
        Select::Zigbuild => {
            select_cmd("是否安装 cargo-zigbuild 构建工具? [y/n]", "已取消安装 zigbuild");
            if let Ok(_) = zigbuild().await {}else { println!("zigbuild 添加失败")}; std::process::exit(0)
        }
    }
}

/* 帮助 */
pub fn help(){
    /* 定义命令参数集合 */
    let cmd = vec![
        "无参数 , 安装 rustup 并设置镜像",
        "h , 帮助 ",
        "v , 版本号",
        "c , 代码仓库",
        "cargo , 添加 cargo 镜像",
        "install-nightly , 安装 rust nightly 版本",
        "install-stable , 安装 rust stable 版本(默认)",
        "list , 列出所有 rust 版本",
        "nightly , 切换到 rust nightly 版本",
        "remove-nightly , 删除 rust nightly 版本",
        "remove-zigbuild , 删除 zigbuild 构建工具",
        "stable , 切换到 rust stable 版本",

        #[cfg(target_os = "linux")]
        "tap , 开启 fish 的 tap 补全",

        "uninstall , 删除 rust",
        "update , 更新 rust",
        "zigbuild , 添加 zigbuild 构建工具",
    ];

    /* 打印参数命令信息 */
    for i in cmd { println!("{i}") } std::process::exit(0)
}

/* 代码仓库跳转 */
pub async fn jump() -> Result<(), Box<dyn std::error::Error>> {
    /* 跳转代码仓库 */
    #[cfg(target_os = "linux")]
    cmd(r#"xdg-open https://gitcode.com/songjiaqicode/rust-installation"#).await?;
    /* 这会调用 xdg-open(桌面通用web接口) 打开代码仓库 */

    #[cfg(target_os = "windows")]
    Command::new("start").args([r#""#,r#"https://gitcode.com/songjiaqicode/rust-installation"#]).status().await?;

    /* 打印代码仓库 */
    println!("gitcode:\nhttps://gitcode.com/songjiaqicode/rust-installation\ngitee:\nhttps://gitee.com/songjiaqicode/rust-installation"); std::process::exit(0)
}

/* 添加 cargo 镜像<1.68版本以上> */
pub async fn cargo() -> Result<(), Box<dyn std::error::Error>> {
    /* cargo 镜像存在性检测 */
    let cargo_path = if let Ok(e) = cargo_bool().await{e}else { println!("cargo 镜像已存在,无需添加"); std::process::exit(0) };

    /* 确认rust版本 */
    select_cmd(
        "\n如果你的rust版本低于1.68,或者未安装rust,切勿添加镜像,否则会报错☠️\n\n执行 cargo -V&&rustc -V 命令来确认 rust 版本以及是否安装\n\n你是否已安装 rust 并且版本大于或等于 1.68? [y/n]",
        "已取消 cargo 镜像添加"
    );

    /* 定义镜像 https 链接 */
    let cargo_https =  r#"
[source.crates-io]
replace-with = 'mirror'

[source.mirror]
registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"

[registries.mirror]
index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
    "#;

    /* 添加镜像 */
    println!("\n镜像内容:\n{}\n",cargo_https);
    let mut file = if let Ok(e) = OpenOptions::new().append(true).create(true).open(&cargo_path).await{e}else { println!("$HOME/.cargo/config.toml 创建失败"); std::process::exit(0) };
    let _ = file.write_all(cargo_https.as_bytes()).await?;
    println!("镜像添加成功,可执行 nano $HOME/.cargo/config.toml 命令查看\n若发现重复镜像可放心删除");

    Ok(())
}

/* 安装 rust nightly 版本 */
pub async fn install_nightly() -> Result<(), Box<dyn std::error::Error>> {
    /* 安装 rust nightly 版本 */
    cmd(r#"RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup rustup install nightly"#).await?;


    Ok(())
}

/* 安装 rust stable 版本 */
pub async fn install_stable() -> Result<(), Box<dyn std::error::Error>> {
    /* 安装 rust stable 版本 */
    cmd(r#"RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup rustup install stable"#).await?;

    Ok(())
}

/* 列出所有 rust 版本 */
pub async fn list() -> Result<(), Box<dyn std::error::Error>> {
    /* 列出所有 rust 版本 */
    cmd(r#"rustup show"#).await?;

    Ok(())
}

/* 切换 rust nightly 版本 */
pub async fn nightly() -> Result<(), Box<dyn std::error::Error>> {
    /* 切换 rust nightly 版本 */
    cmd(r#"rustup default nightly"#).await?;

    Ok(())
}

/* 删除 rust nightly 版本 */
pub async fn remove_nightly() -> Result<(), Box<dyn std::error::Error>> {
    cmd(r#"rustup toolchain uninstall nightly"#).await?;
    Ok(())
}

/* 删除 zigbuild 构建工具 */
pub async fn remove_zigbuild() -> Result<(), Box<dyn std::error::Error>> {
    /* 删除 zigbuild */
    cmd(r#"cargo uninstall cargo-zigbuild"#).await?;

    Ok(())
}

/* 切换 rust stable 版本 */
pub async fn stable() -> Result<(), Box<dyn std::error::Error>> {
    cmd(r#"rustup default stable"#).await?;
    Ok(())
}

/* fish 的 tap 补全 */
#[cfg(target_os = "linux")]
pub async fn tap() -> Result<(), Box<dyn std::error::Error>> {
    /* 判断 fish 存在性 */
    let res = Command::new("fish").arg("-v").stdout(Stdio::null()).stderr(Stdio::null()).spawn();
    if let Ok(_) = res { println!("fish存在,正在创建配置文件") } else { println!("fish不存在"); std::process::exit(0) };

    /* 定义 fish tap 文件 */
    let file = OpenOptions::new().read(true).write(true).create(true).open(res_path(".config/fish/completions/rust-install.fish")).await;

    /* 判断创建正确性 */
    let mut file = if let Ok(e) = file { println!("fish tap 文件创建成功"); e }else { println!("fish tap 文件创建失败"); std::process::exit(0) };

    /* 定义写入内容 */
    let write = b"complete -c rust-install -f -a 'h v c cargo install-nightly install-stable list nightly remove-nightly remove-zigbuild stable tap uninstall update zigbuild'";

    /* 写入文件 */
    let _ = file.write_all(write).await?;

    /* 立刻激活 */
    Command::new("fish").arg("-c").arg(r#"source $HOME/.config/fish/completions/rust-install.fish"#).spawn()?;

    Ok(())
}

/* 删除 uninstall */
pub async fn uninstall() -> Result<(), Box<dyn std::error::Error>> {
    /* 删除 uninstall */
    cmd(r#"rustup self uninstall"#).await?;

    Ok(())
}

/* 更新 rust */
pub async fn update() -> Result<(), Box<dyn std::error::Error>> {
    /* 更新 rust */
    cmd(r#"rustup update"#).await?;

    Ok(())
}

/* 添加 zigbuild 构建工具 */
pub async fn zigbuild() -> Result<(), Box<dyn std::error::Error>> {
    /* 安装 zigbuild */
    cmd(r#"cargo install --locked cargo-zigbuild"#).await?;

    Ok(())
}

/* 路径处理 */
fn res_path(path:&str) -> PathBuf {
    /* 获取 $HOME 环境变量并转换为 $Path */
    #[cfg(target_os = "linux")]
    let home = var("HOME").expect("linux 解包失败");

    #[cfg(target_os = "windows")]
    let home = var("USERPROFILE").expect("windows 解包失败");

    let home = Path::new(&home);
    /* join() 不接受 String 所以转化为 &Path */

    /* 路径合并 */
    let path = home.join(path);

    /* 返回合并后的路径 */
    path
}

/* cargo 镜像存在性检测 */
async fn cargo_bool() -> Result<PathBuf, Box<dyn std::error::Error>> {
    /* 定义 .cargo 配置文件路径 */
    let cargo_path = res_path(".cargo/config.toml");

    /* 打开 .cargo/config.toml 配置文件 */
    if let Ok(mut e) = OpenOptions::new().read(true).open(&cargo_path).await {

        /* 读取 config.toml 配置文件内容 */
        let mut buf = String::new();
        let _ = e.read_to_string(&mut buf).await;
        /* buf 存放了 config.toml 配置文件内容 */

        /* 定义镜像关键部分 */
        let https = vec![
            "[source.crates-io]",
            "replace-with = 'mirror'",
            "[source.mirror]",
            r#"registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/""#,
            "[registries.mirror]",
            r#"index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/""#
        ];

        /* 代表存在性的变量 */
        let mut all = true;
        /* 默认完整存在 */

        /* 遍历镜像关键部分 */
        for i in https {
            /* 判断 buf 是否完整包含 https 的成员切片  */
            if !buf.contains(i) {
                /* 不包含处理 */
                all = false; break;
                /* 将存在性变量改为 false 并退出遍历 */
            }
        }

        /* 根据存在性变量执行对应的操作 */
        if all { return Err("镜像已存在".into()) }
        /* true 返回值 , false 返回错误 */
    };

    /* 返回 .cargo 配置文件路径 */
    Ok(cargo_path)
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
    Command::new(shell)
        .env("RUSTUP_UPDATE_ROOT","https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup")
        .env("RUSTUP_DIST_SERVER","https://mirrors.tuna.tsinghua.edu.cn/rustup")
        .status().await?;

    Ok(())
}

/* 通用选择 */
fn select_cmd(pr:&str, rn:&str) {
    /* 安装选择 */
    println!("{}",pr);
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);

    /* 判断选择 */
    if buf.trim() == "y" { () }else { println!("{}",rn); std::process::exit(0); }
}