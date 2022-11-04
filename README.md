# SS

a rust version ss
> [ss](https://github.com/alexzhang1030/ss) ss means "Search, Search"
Just type something, ss will open corresponding website with your input keywords in your default browser.

## Usage

### Basic Usage

```bash
ss [...keywords]

# such like this
ss javascript array reduce # <- will open google.com with query "javascript array reduce"
```

### Specific Website

```bash
ss -u <website-type> [...keywords]

# such like
ss -u git alexzhang1030 # <- will open github.com with query "alexzhang1030"
```

## Config

in development

## Default Support Website

```json
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

## License

MIT, hildxd
