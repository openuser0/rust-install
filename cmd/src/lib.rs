//! #库包入口 ( lib.rs )

//! ##功能

//! 每一个 库包 的 入口 , 可以在 这里 定义 公开接口

//! ##作者

//! songjiaqicode

//! ##初始化日期

//! 2025.7.11

/* 注册模块 , 让 编译器 识别 库包内 的 模块 */

mod repl_trait;/* 接口操作 模块 */

mod repl_struct;/* 类型操作 模块 */

mod repl_function;/* 函数操作 模块 */

/* 暴露 公共 接口 */

/* 调用 接口操作 */

pub fn rl_trait(){

    /* 执行 repl_trait 内部操作 */
    repl_trait::run();
}

/* 调用 类型操作 */

pub fn rl_struct(){

    /* 执行 repl_struct 内部操作 */
    repl_struct::run();
}

/* 调用 函数操作 */

pub async fn rl_function(){

    /* 执行 repl_function 内部操作 */
    let _ = repl_function::run().await;
}