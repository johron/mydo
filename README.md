# mydo
my first rust project so this is probably not very efficient and it uses a lot of large crates so i makes very large binaries.

## Features
- run files without needing to put in args every time
- run files with shebangs
- initialize project easily with presets
  - can be local, network downloaded or from commands

## Help
```bash
mydo help
```

## Install
### Script
```bash
curl -fsSL http://sh.johanrong.me/mydo/install.sh | bash
```

### Compile from source
#### build debug 
```bash
git clone https://github.com/johron/mydo.git
cd mydo
cargo build
```
#### build release
```bash
git clone https://github.com/johron/mydo.git
cd mydo
cargo build --release
```

# Example Configuartion
#### ~/.mydo/mydo.json
```json
{
  "settings": {
    "show_time": true
  },
  "inits": {
    "python3": "{home}/.mydo/inits/python3.tar",
    "node": "{home}/.mydo/inits/node.tar",
    "typescript": "https://example.com/inits/ts.tar",
    "rust": "{home}/.cargo/bin/cargo init"
  }
}
```

#### ./mydo.json
```json
{
  "presets": {
    "run": "{home}/.cargo/bin/cargo run",
    "build": "{home}/.cargo/bin/cargo build",
    "js": "/usr/bin/node {file}"
  }
}

```