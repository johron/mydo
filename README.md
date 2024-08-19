# **This project has been replaced with:** https://github.com/johron/mydo

# Mydo
Easy project manager and runner written in rust.
(this project is dead. It works, but definetly has some problems and is not recommended to be dependent on)
Had some trouble reverting the commits so ended up deleting repo and pushing older commit so the are gone

My first rust project so this is probably not very efficient and it uses a lot of large crates so i makes very large binaries.

## Features
- Run files without needing to put in args every time
- Run files with shebangs
- Initialize project easily with presets
  - Can be local, network downloaded or from commands

## Help
```bash
mydo --help
```

### Manual: Compile from source
- File structure
  - `~/.mydo/`
    - The `mydo.json` main configuration file located here
  - `~/.mydo/inits/`
    - Tar archives with initializations here
  - `~/.mydo/bin/`
    - The `mydo` binary located here

#### Build binary
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
  "settings": {
    "show_time": false,
    "root": "build"
  },
  "run": "{home}/.cargo/bin/cargo run",
  "build": "{home}/.cargo/bin/cargo build",
  "runners": {
    "js": "/usr/bin/node {file}"
  }
}

```
