use std::default;

use anyhow::{Context, Ok, Result};
use dirs;
// use serde_yaml;
use linked_hash_map::LinkedHashMap;
use tokio::{fs::File, io::AsyncReadExt};
use yaml_rust::{Yaml, YamlLoader};

pub fn init_config() -> LinkedHashMap<String, String> {
    let mut map = LinkedHashMap::new();
    map.insert(
        "baidu".to_string(),
        "https://www.baidu.com/s?wd={keyword}".to_string(),
    );
    map.insert(
        "google".to_string(),
        "https://www.google.com/search?q={keyword}".to_string(),
    );
    map.insert(
        "mdn".to_string(),
        "https://developer.mozilla.org/zh-CN/search?q={keyword}".to_string(),
    );
    map.insert(
        "npm".to_string(),
        "https://www.npmjs.com/search?q={keyword}".to_string(),
    );
    map.insert(
        "git".to_string(),
        "https://github.com/search?q={keyword}".to_string(),
    );
    map.insert(
        "default".to_string(),
        "https://www.google.com/search?q={keyword}".to_string(),
    );
    map
}

pub async fn read_config() -> Result<LinkedHashMap<String, String>> {
    let path = dirs::home_dir().unwrap();
    let path = path.join(".ss.yaml");
    let mut map = LinkedHashMap::new();
    let f = File::open(path)
        .await
        .with_context(|| "open config file error");
    if let Result::Ok(mut file) = f {
        let mut content = String::new();
        file.read_to_string(&mut content)
            .await
            .with_context(|| "read config file error")?;
        let doc = YamlLoader::load_from_str(&content).with_context(|| "parse config file error")?;
        read_to_yaml(&doc[0], &mut map);
    };
    Ok(map)
}

fn read_to_yaml(doc: &Yaml, map: &mut LinkedHashMap<String, String>) -> Result<()> {
    let default = doc["default"].as_str();
    if let Some(default) = default {
        map.insert("default".into(), default.into());
    }
    let extends = doc["extend"].as_vec();
    if let Some(extends) = extends {
        extends.iter().for_each(|x| {
            let key = x["name"]
                .as_str()
                .with_context(|| "key is not string")
                .unwrap();
            let value = x["rule"]
                .as_str()
                .with_context(|| "value is not string")
                .unwrap();
            map.insert(key.into(), value.into());
        });
    }
    Ok(())
}
