# RSS

a rust version ss
> [ss](https://github.com/alexzhang1030/ss) ss means "Search, Search"
Just type something, ss will open corresponding website with your input keywords in your default browser.

## install

```bash
cargo install r-ss
```

## Usage

### Basic Usage

```bash
r-ss [...keywords]

# such like this
r-ss javascript array reduce # <- will open google.com with query "javascript array reduce"
```

### Specific Website

```bash
r-ss -u <website-type> [...keywords]

# such like
r-ss -u git alexzhang1030 # <- will open github.com with query "alexzhang1030"
```

## Config

config file in `~/.ss.yaml`  
example:

```yaml
# set default website [google, baidu]
default: google
# set extends website
extend:
  - name: bilibili
    rule: https://search.bilibili.com/all?keyword={keyword}
```

## Default Support Website

```js
[
  {
    name: 'baidu',
    rule: 'https://www.baidu.com/s?wd={keyword}',
  },
  {
    name: 'google',
    rule: 'https://www.google.com/search?q={keyword}',
  },
  {
    name: 'mdn',
    rule: 'https://developer.mozilla.org/zh-CN/search?q={keyword}',
  },
  {
    name: 'npm',
    rule: 'https://www.npmjs.com/search?q={keyword}',
  },
  {
    name: 'git',
    rule: 'https://github.com/search?q={keyword}',
  },
]
```
