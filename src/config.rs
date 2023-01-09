use std::collections::HashMap;
use reqwest::{Method};
use reqwest::header::HeaderMap;
use url::Url;
use serde::{Serialize, Deserialize};
use tokio::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffConfig {
    #[serde(flatten)]   //  这里flatten的作用是把yml文件最外层的profiles去掉，不用再在最外层添加这个"profiles:"
    pub profiles: HashMap<String, DiffProfile>,
}

// 这里的字段名要跟yml的对上
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,

    // 这个ResponseProfile，可以跳过某些内容，例如两次访问首页，可以有不一样的内容
    pub res: ResponseProfile,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestProfile {
    // 这里要用http_serde来序列化；default的作用是默认用Method这个结构体实现的Default方法，而现在这个Default就是Method::GET
    #[serde(with = "http_serde::method", default)]
    pub method: Method,

    // 因为http_serde不支持Url的序列化，而Url库的Url自带序列化，所以用Url库
    // 注意这里的Url不是reqwest里的Url
    pub url: Url,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub params: Option<serde_json::Value>,      // params要可以重复，所以不能用HashMap

    #[serde(skip_serializing_if = "HeaderMap::is_empty", with = "http_serde::header_map", default)]
    pub headers: HeaderMap,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseProfile{
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

pub struct DiffArgs {

}

impl DiffConfig {
    pub async fn load_yaml(path: &str) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }

    pub fn from_yaml(content: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_str(content)?)
    }

    pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
        self.profiles.get(name)
    }
}

impl DiffProfile {
    // 接收命令行传过来的参数给到DiffArgs；然后再send
    pub async fn diff(&self, _args: DiffArgs) -> anyhow::Result<String> {
        // let res1 = req1.send(&args).await?;
        // let res2 = req2.send(&args).await?;
        //
        // let text1 = res1.filter_text(&self.res).await?;
        // let text2 = res2.filter_text(&self.res).await?;
        //
        // text_diff(&text1, &text2);

        todo!()
    }
}