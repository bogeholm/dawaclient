### Rust kodeeksempel: Klientapplikation til Danmarks Adressers Web API - DAWA

[![Travis build](https://travis-ci.com/bogeholm/dawaclient.svg?branch=master)](https://travis-ci.com/github/bogeholm/dawaclient)
[![dependency status](https://deps.rs/repo/github/bogeholm/dawaclient/status.svg)](https://deps.rs/repo/github/bogeholm/dawaclient)

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

#### Compile med optimering og kør
```bash
$ cargo build --release
$ cd target/release
$ ./dawaclient Vestergade 1
``` 
