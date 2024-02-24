# do-rs
- my first rust project so this is probably not very efficient
- and it uses a lot of large crates so i makes very large
- binaries

## Features
- run files without needing to put in args every time
- run files with shebangs
- initialize project easily with presets
  - can be local, network downloaded or from commands

## Help
```bash
do help
```

## Install
### Script
- Install script comming soon, for now download from releases or compile from source.

### Compile from source
#### build debug 
```bash
git clone https://github.com/johanrong/do-rs.git
cd do-rs
cargo build
```
#### build release
```bash
git clone https://github.com/johanrong/do-rs.git
cd do-rs
cargo build --release
```

# Example Configuartion
#### ~/.do-rs/do.json
```json
{
  "settings": {
    "show_time": true
  },
  "presets": {
    "py": "/usr/bin/python3 {file}",
    "js": "/usr/bin/node {file}",
    "c": "/usr/bin/gcc {file} -o {output}"
  },
  "inits": {
    "python": "{home}/python3.tar",
    "rust": "{home}/.cargo/bin/cargo init",
    "node": "https://example.com/node.tar"
  }
}
```
- Can have all keys

#### ./do.json
```json
{
  "presets": {
    "run": "{home}/.cargo/bin/cargo run",
    "build": "{home}/.cargo/bin/cargo build",
    "py": "/usr/bin/python3 {file}",
    "js": "/usr/bin/node {file}"
  }
}

```
- Cannot have "settings" or "inits" keys in this file. Only "presets" key is valid
- "run" and "build" in presets are only for this file, I think, they should only be used here
