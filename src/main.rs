use func;

use cmd;

#[tokio::main]
async fn main(){

    /* 调用接口 */
    cmd::rl_trait();

    cmd::rl_struct();

    #[cfg(target_os = "linux")]
    cmd::rl_function().await;

    func::rl_trait();

    func::rl_struct().await;

    #[cfg(target_os = "linux")]
    func::rl_function().await;
}