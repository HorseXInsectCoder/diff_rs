
use clap::Parser;

// 实现了Parser宏，就能被clap解析
// 比较两个请求的返回
#[derive(Parser, Debug, Clone)]
#[clap(version, author, about, long_about = None)] // 单独加上clap参数，如果没有不给定的话，会从toml里拿
pub(crate) struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

// 命令要可扩展
#[derive(Subcommand, Debug, Clone)]
enum Action {

}