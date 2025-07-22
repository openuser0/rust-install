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
use tokio::io::{AsyncWriteExt, AsyncReadExt};
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
    /* 匹配枚举值 */
    match par {
        /* 帮助 */
        Select::H =>{ help() }

        /* 版本号 */
        Select::V =>{ println!("1.4.0"); std::process::exit(0) }

        /* 代码仓库 */
        Select::C => { if let Ok(_) = jump().await {}else { println!("代码仓库跳转失败")}; std::process::exit(0)}

        /* 列出rust版本信息 */
        Select::List => { select_cmd("是否列出 rust 版本信息? [y/n]"); if let Ok(_) = list().await {}else { println!("失败☠️") }; std::process::exit(0) }

        /* 添加 cargo 镜像 */
        Select::Cargo => { select_cmd("是否添加 cargo 镜像? [y/n]"); if let Ok(_) = cargo().await {}else { println!("失败☠️") }; std::process::exit(0) }

        /* 开启fish shell tap补全 */
        Select::TapFish => { select_cmd("是否开启fish shell tap补全? [y/n]"); if let Ok(_) = tap_fish().await {}else { println!("失败☠️") }; std::process::exit(0) }

        /* 开启bash shell tap补全 */
        Select::TapBash => { select_cmd("是否开启bash shell tap补全? [y/n]"); println!("暂无"); std::process::exit(0) }

        /* 安装 rust nightly 版本 */
        Select::InstallNightly => { select_cmd("是否安装 rust nightly 版本? [y/n]"); if let Ok(_) = install_nightly().await {}else { println!("失败☠️") }; std::process::exit(0) }

        /* 切换到 nightly 版本 */
        Select::Nightly => { select_cmd("是否切换到 nightly 版本? [y/n]"); if let Ok(_) = nightly().await {} else { println!("失败☠️") } std::process::exit(0) }

        /* 删除 rust nightly 版本 */
        Select::RemoveNightly => { select_cmd("是否删除 rust nightly 版本? [y/n]"); if let Ok(_) = remove_nightly().await {} else { println!("失败☠️") }; std::process::exit(0) }

        /* 切换到 stable 版本 */
        Select::Stable => { select_cmd("是否切换到 stable 版本? [y/n]"); if let Ok(_) = stable().await {} else { println!("失败☠️") }; std::process::exit(0) }

        /* 删除 rust */
        Select::Uninstall => { select_cmd("是否删除 rust? [y/n]"); if let Ok(_) = uninstall().await {}else { println!("失败☠️") }; std::process::exit(0) }

        /* 更新 rust */
        Select::Update => { select_cmd("是否更新 rust? [y/n]"); if let Ok(_) = update().await {}else { println!("失败☠️") }; std::process::exit(0) }

        /* 添加 zigbuild 构建工具 */
        Select::Zigbuild => { select_cmd("是否添加 cargo-zigbuild 构建工具? [y/n]"); if let Ok(_) = zigbuild().await {}else { println!("失败☠️") }; std::process::exit(0) }

        /* 文档 */
        Select::DocZigbuild => { select_cmd("是否查看 cargo-zigbuild 文档? [y/n]"); println!("暂无"); std::process::exit(0) }

        /* 删除 zigbuild 构建工具 */
        Select::RemoveZigbuild => { select_cmd("是否删除 cargo-zigbuild 构建工具? [y/n]"); if let Ok(_) = remove_zigbuild().await {}else { println!("失败☠️") }; std::process::exit(0) }

        /* 添加tauri框架 */
        Select::Tauri => { select_cmd("是否添加 tauri 框架? [y/n]"); println!("暂无"); std::process::exit(0) }

        /* 文档 */
        Select::DocTauri => { select_cmd("是否查看 tauri 文档? [y/n]"); println!("暂无"); std::process::exit(0) }

        /* 删除tauri */
        Select::RemoveTauri => { select_cmd("是否删除 tauri 框架? [y/n]"); println!("暂无"); std::process::exit(0) }
    }
}

/* 帮助 */
pub fn help(){
    /* 定义命令参数集合 */
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

    /* 打印参数命令信息 */
    for i in cmd { println!("{i}") } std::process::exit(0)
}

/* 代码仓库跳转 */
pub async fn jump() -> Result<(), Box<dyn std::error::Error>> {
    /* 跳转代码仓库 */
    #[cfg(target_os = "linux")]
    cmd("xdg-open https://gitcode.com/songjiaqicode/rust-installation").await?;

    #[cfg(target_os = "windows")]
    Command::new("cmd").args(["/C","start","https://gitcode.com/songjiaqicode/rust-installation"]).status().await?;

    /* 打印代码仓库 */
    println!("gitcode:\nhttps://gitcode.com/songjiaqicode/rust-installation\ngitee:\nhttps://gitee.com/songjiaqicode/rust-installation"); std::process::exit(0)
}

/* 添加 cargo 镜像<1.68版本以上> */
pub async fn cargo() -> Result<(), Box<dyn std::error::Error>> {
    /* cargo 镜像存在性检测 */
    let cargo_path = if let Ok(e) = cargo_bool().await{e}else { println!("cargo 镜像已存在,无需添加"); std::process::exit(0) };
    select_cmd("\n如果你的rust版本低于1.68,或者未安装rust,切勿添加镜像,否则会报错☠️\n\n执行 cargo -V&&rustc -V 命令来确认 rust 版本以及是否安装\n\n你是否已安装 rust 并且版本大于或等于 1.68? [y/n]");

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
    println!("镜像添加成功");

    Ok(())
}

/* 安装 rust nightly 版本 */
pub async fn install_nightly() -> Result<(), Box<dyn std::error::Error>> { cmd("RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup rustup install nightly").await?; Ok(()) }

/* 列出所有 rust 版本 */
pub async fn list() -> Result<(), Box<dyn std::error::Error>> { cmd("rustup show").await?; Ok(()) }

/* 切换 rust nightly 版本 */
pub async fn nightly() -> Result<(), Box<dyn std::error::Error>> { cmd("rustup default nightly").await?; Ok(()) }

/* 删除 rust nightly 版本 */
pub async fn remove_nightly() -> Result<(), Box<dyn std::error::Error>> { cmd("rustup toolchain uninstall nightly").await?; Ok(()) }

/* 删除 zigbuild 构建工具 */
pub async fn remove_zigbuild() -> Result<(), Box<dyn std::error::Error>> { cmd("cargo uninstall cargo-zigbuild").await?; Ok(()) }

/* 切换 rust stable 版本 */
pub async fn stable() -> Result<(), Box<dyn std::error::Error>> { cmd("rustup default stable").await?; Ok(()) }

/* fish 的 tap 补全 */
pub async fn tap_fish() -> Result<(), Box<dyn std::error::Error>> {
    /* 判断 fish 存在性 */
    let res = Command::new("fish").arg("-v").status().await;
    if let Ok(_) = res { println!("fish存在,正在创建配置文件") } else { println!("fish不存在"); return Err("fish不存在".into()) };

    /* 创建 fish tap 补全文件 */
    let file = OpenOptions::new().read(true).write(true).create(true).open(res_path(".config/fish/completions/rust-install.fish")).await;
    let mut file = if let Ok(e) = file { println!("fish tap 文件创建成功"); e }else { println!("fish tap 文件创建失败"); std::process::exit(0) };

    /* 写入内容 */
    let write = b"complete -c rust-install -f -a 'h v c list cargo tap-fish tap-bash install-nightly nightly remove-nightly stable uninstall update zigbuild doc-zigbuild remove-zigbuild tauri doc-taur remove-tauri'";
    let _ = file.write_all(write).await?; println!("重启终端后即可使用 fish tap补全");

    Ok(())
}

/* 删除 uninstall */
pub async fn uninstall() -> Result<(), Box<dyn std::error::Error>> { cmd(r#"rustup self uninstall"#).await?; Ok(()) }

/* 更新 rust */
pub async fn update() -> Result<(), Box<dyn std::error::Error>> { cmd("rustup update").await?; Ok(()) }

/* 添加 zigbuild 构建工具 */
pub async fn zigbuild() -> Result<(), Box<dyn std::error::Error>> { cmd("cargo install --locked cargo-zigbuild").await?; Ok(()) }

/* 路径处理 */
fn res_path(path:&str) -> PathBuf {
    /* 获取家目录环境变量并转换为 $Path */
    #[cfg(target_os = "linux")]
    let home = var("HOME").expect("linux 解包失败");

    #[cfg(target_os = "windows")]
    let home = var("USERPROFILE").expect("windows 解包失败");

    /* 合并路径 */
    let home = Path::new(&home);
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

        /* 镜像关键部分存在性判断 */
        for i in https { if !buf.contains(i) { all = false; break; } }

        /* 根据存在性变量执行对应的操作 */
        if all { return Err("镜像已存在".into()) }
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
    Command::new(r#"C:\msys64\usr\bin\bash.exe"#).args(["-c",shell])
        .env("RUSTUP_UPDATE_ROOT","https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup")
        .env("RUSTUP_DIST_SERVER","https://mirrors.tuna.tsinghua.edu.cn/rustup")
        .status().await?;

    Ok(())
}

/* 通用选择 */
fn select_cmd(pr:&str) {
    /* 安装选择 */
    println!("{}",pr);
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);

    /* 判断选择 */
    if buf.trim() == "y" { () }else { println!("操作已取消"); std::process::exit(0); }
}