### Rust kodeeksempel: Klientapplikation til Danmarks Adressers Web API - DAWA

[![Travis build](https://travis-ci.com/bogeholm/dawaclient.svg?branch=master)](https://travis-ci.com/github/bogeholm/dawaclient)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://en.wikipedia.org/wiki/Software_license)
[![dependency status](https://deps.rs/repo/github/bogeholm/dawaclient/status.svg)](https://deps.rs/repo/github/bogeholm/dawaclient) ([deps.rs issue #41](https://github.com/srijs/deps.rs/issues/41))

### Guide
[Installer Rust](https://www.rust-lang.org/tools/install), og derefter:
```bash
$ git clone git@github.com:bogeholm/dawaclient.git
$ cd dawaclient
```

#### Dokumentation
```bash
$ cargo doc --no-deps --open
```

#### [Quick](https://blog.mozilla.org/nnethercote/2019/10/11/how-to-speed-up-the-rust-compiler-some-more-in-2019/)start
```bash
$ cargo run -- Vestergade 1
``` 

#### Compile med optimeringer
```bash
$ cargo build --release
$ cd target/release
$ ./dawaclient Vestergade 1
``` 

### Docker
```shell
$ docker build --tag dawars --file Docker/Dockerfile .
$ docker run -it --rm dawars
root@<container ID>:/# dawaclient Vestergade 1
```
Se [dockerhub/rust](https://hub.docker.com/_/rust)

# License

This project is licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.