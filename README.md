# Mydo
Easy project manager and runner written in rust.

My first rust project so this is probably not very efficient and it uses a lot of large crates so i makes very large binaries.

## Features
- Run files without needing to put in args every time
- Run files with shebangs
- Initialize project easily with presets
  - Can be local, network downloaded or from commands

## Help
```bash
mydo help
```

## Installation
### Automatic: Install script
```bash
curl -fsSL http://sh.johanrong.me/mydo/install.sh | bash
```

### Manual: Compile from source
- File structure
  - `~/.mydo/`
    - The `mydo.json` main configuration file located here
  - `~/.mydo/inits/`
    - Tar archives with initializations here
  - `~/.mydo/bin/`
    - The `mydo` binary located here

#### Build a debug version
```bash
git clone https://github.com/johron/mydo.git
cd mydo
cargo build
```
#### Build a release version
```bash
git clone https://github.com/johron/mydo.git
cd mydo
cargo build --release
```

## Example Configuartion
#### Mydo main configuration
`~/.mydo/mydo.json`
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

#### Mydo project specific configuration
`./mydo.json`
```json
{
  "run": "{home}/.cargo/bin/cargo run",
  "build": "{home}/.cargo/bin/cargo build",
  "runners": {
    "js": "/usr/bin/node {file}"
  }
}

```