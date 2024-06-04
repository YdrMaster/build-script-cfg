# buil-script-cfg

[![CI](https://github.com/YdrMaster/build-script-cfg/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/YdrMaster/build-script-cfg/actions)
[![Latest version](https://img.shields.io/crates/v/build-script-cfg.svg)](https://crates.io/crates/build-script-cfg)
[![Documentation](https://docs.rs/build-script-cfg/badge.svg)](https://docs.rs/build-script-cfg)
[![license](https://img.shields.io/github/license/YdrMaster/build-script-cfg)](https://mit-license.org/)
![GitHub repo size](https://img.shields.io/github/repo-size/YdrMaster/build-script-cfg)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/YdrMaster/build-script-cfg)

[![GitHub Issues](https://img.shields.io/github/issues/YdrMaster/build-script-cfg)](https://github.com/YdrMaster/build-script-cfg/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/YdrMaster/build-script-cfg)](https://github.com/YdrMaster/build-script-cfg/pulls)
![GitHub contributors](https://img.shields.io/github/contributors/YdrMaster/build-script-cfg)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/YdrMaster/build-script-cfg)

Configure and set cfg flags in build scripts.

## Usage

```rust,ignore
let cfg = Cfg::new("cfg");
if some_condition {
    cfg.define();
}
```
