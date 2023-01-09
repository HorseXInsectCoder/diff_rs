use anyhow::Result;
use diff_rs::DiffConfig;

fn main() -> Result<()> {
    let content = include_str!("../fixtures/test.yml");
    let config = DiffConfig::from_yaml(content)?;

    println!("{:#?}", config);

    Ok(())
}