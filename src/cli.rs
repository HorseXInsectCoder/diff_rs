
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
pub(crate) enum Action {
    /// Diff two API response based on given profile
    Run(RunArgs),

}

pub(crate) struct RunArgs {
    /// profile name
    #[clap(short, long, value_parser)]      // 如果不给定value_parser的话，就会用default parser把参数原原本本地解析为String
    pub profile: String,

    /// overrides args.Could be used to override the query, headers and body of the request.
    #[clap(short, long, value_parser = parse_key_val, number_of_value = 1)] // number_of_value = 1让每个k-v参数都要在前面加上 "-x"
    pub extra_params: Vec<KeyVal>
}

#[derive(Debug, Clone)]
pub(crate) struct KeyVal {
    key: String,
    value: String,
}