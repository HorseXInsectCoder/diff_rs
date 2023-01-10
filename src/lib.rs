mod config;
pub(crate) mod cli;      // 表示crate内的数据结构，只在crate内使用


pub use config::{DiffConfig, DiffProfile, RequestProfile, ResponseProfile};

