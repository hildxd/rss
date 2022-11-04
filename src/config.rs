use std::collections::HashMap;

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
