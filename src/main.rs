use func;

use cmd;

#[tokio::main]
async fn main(){

    /* 调用接口 */
    cmd::rl_trait();

    cmd::rl_struct();

    cmd::rl_function().await;

    func::rl_trait();

    func::rl_struct().await;

    func::rl_function().await;
}