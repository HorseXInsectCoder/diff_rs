mod config;
pub(crate) mod cli;      // 表示crate内的数据结构，只在crate内使用


pub use config::{DiffConfig, DiffProfile, RequestProfile, ResponseProfile};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExtraArgs {
    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub body: Vec<(String, String)>,
}