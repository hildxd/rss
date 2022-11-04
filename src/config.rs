use anyhow::{anyhow, Context, Ok, Result};
use dirs;
use serde_yaml;
use std::collections::HashMap;
use tokio::{fs::File, io::AsyncReadExt};

pub fn init_config<'a>() -> HashMap<&'a str, &'a str> {
    let mut map = HashMap::new();
    map.insert("baidu", "https://www.baidu.com/s?wd={keyword}");
    map.insert("google", "https://www.google.com/search?q={keyword}");
    map.insert(
        "mdn",
        "https://developer.mozilla.org/zh-CN/search?q={keyword}",
    );
    map.insert("npm", "https://www.npmjs.com/search?q={keyword");
    map.insert("git", "https://github.com/search?q={keyword}");
    map
}

// #[tokio::main]
pub async fn read_config() -> Result<()> {
    let path = dirs::home_dir().unwrap();
    let path = path.join(".ss.yaml");
    let f = File::open(path)
        .await
        .with_context(|| "open config file error");
    if let Result::Ok(mut file) = f {
        let mut content = String::new();
        file.read_to_string(&mut content).await.with_context(|| "read config file error")?;
        // let map: serde_yaml::Value = serde_yaml::from_str(&content)?;
        // println!(
        //     "Read YAML String:{}",
        //     map["default"]
        //         .as_str()
        //         .map(|s| s.to_string())
        //         .ok_or(anyhow!("parse config file error"))?
        // );
    }
    Ok(())
}
