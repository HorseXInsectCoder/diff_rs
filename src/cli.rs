
use clap::{Parser, Subcommand};
use anyhow::{Result, anyhow};
use crate::ExtraArgs;

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

#[derive(Parser, Debug, Clone)]
pub(crate) struct RunArgs {
    /// profile name
    #[clap(short, long, value_parser)]      // 如果不给value_parser一个函数的话（下面的字段就给了value_parser一个函数），就会用default parser把参数原原本本地解析为String
    pub profile: String,

    /// overrides args.Could be used to override the query, headers and body of the request.
    /// For query params, use '-e &key=value'
    /// For body, use '-e @key=value'
    #[clap(short, long, value_parser = parse_key_val, number_of_values = 1)] // number_of_value = 1让每个k-v参数都要在前面加上 "-x" 这样的命令
    pub extra_params: Vec<KeyVal>
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum KeyValType {
    Query,
    Header,
    Body,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct KeyVal {
    key_type: KeyValType,
    key: String,
    value: String,
}

fn parse_key_val(s: &str) -> Result<KeyVal> {
    let mut parts = s.splitn(2, '=');

    // let retrieve = |v: Option<&str>| -> Result<&str> {
    //     Ok(v.ok_or_else(|| anyhow!("Invalid key value pair: {}", s))?
    //         .trim())
    // };

    let key = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid key value pair"))?
        .trim();
    let value = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid key value pair"))?
        .trim();

    let (key_type, key) = match key.chars().next() {
        Some('%') => (KeyValType::Header, &key[1..]),
        Some('@') => (KeyValType::Body, &key[1..]),
        Some(v) if v.is_ascii_alphabetic() => (KeyValType::Query, key),
        _ => return Err(anyhow!("Invalid key value pair")),
    };

    Ok(KeyVal {
        key_type,
        key: key.to_string(),
        value: value.to_string()
    })
}

impl From<Vec<KeyVal>> for ExtraArgs {
    fn from(args: Vec<KeyVal>) -> Self {
        let mut headers = vec![];
        let mut body = vec![];
        let mut query = vec![];

        for arg in args {
            match arg.key_type {
                KeyValType::Header => headers.push((arg.key, arg.value)),
                KeyValType::Body => body.push((arg.key, arg.value)),
                KeyValType::Query => query.push((arg.key, arg.value)),
            }
        }

        Self {
            headers,
            query,
            body
        }
    }
}