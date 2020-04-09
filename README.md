### Rust kodeeksempel: Klientapplikation til Danmarks Adressers Web API - DAWA

[![Travis build](https://travis-ci.com/bogeholm/dawaclient.svg?branch=master)](https://travis-ci.com/github/bogeholm/dawaclient)
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

#### Quickstart
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
[Dokumenation for Rust & Docker](https://hub.docker.com/_/rust)