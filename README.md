# do-rs

# Example Configuartion
#### ~/.do-rs/do.json
```json
{
  "settings": {
    "show_time": true
  },
  "presets": {
    "py": "/usr/bin/python {file}",
    "js": "/usr/bin/node {file}",
    "c": "/usr/bin/gcc {file} -o {output}"
  },
  "inits": {
    "python": "{home}/python3.tar",
    "rust": "{home}/.cargo/bin/cargo init",
    "node": "/usr/bin/npm init"
  }
}
```
- Can have all keys

#### ./do.json
```json
{
  "presets": {
    "py": "/usr/bin/python3 {file}",
    "js": "/usr/bin/node {file}"
  }
}
```
- Cannot have "settings" or "inits" keys in this file. Only "presets" key is valid