<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/edfloreshz/sensei/main/docs/assets/logo.png" width="150" />
  <h1>Sensei (先生)</h1>
  <a href="">
    <img src="https://img.shields.io/github/workflow/status/edfloreshz/sensei/Rust?logo=GitHub" alt="build"/>
  </a>
  <a href="https://crates.io/crates/sensei">
    <img src="https://img.shields.io/crates/v/sensei?label=Sensei" alt="crate"/>
  </a>
  <img src="https://img.shields.io/crates/d/sensei" alt="downloads"/>
  <a href="https://t.me/sensei_rs">
    <img src="https://img.shields.io/static/v1?label=chat&message=Telegram&color=blue&logo=telegram" alt="chat on telegram"/>
  </a>
</div>


  

Sensei is a simple command line tool to open documentation for any crate in crates.io.

## Installation

#### Cargo

```shell
cargo install sensei
```

#### Arch Linux

```rust
yay -S sensei-git
```

## Usage

```rust
ss <crate> [OPTIONS] [FLAGS]
```

### Options

```
-v, --version <version>    Opens documentation for a specific version.
-q, --query <query>      Specifies query to search documentation.
```

### Flags

```
-h, --help      Prints help information
-l, --local    Tries to open local documentation.
-m, --manifest  Looks up the version in Cargo.toml
```

### Examples

##### Opening documentation for a crate.

```rust
ss rand
```

##### Opening local documentation for a crate.

```rust
ss rand -l
ss rand --local
```

##### Specifying a version.

```rust
ss rand -v 0.7.2
ss rand --version 0.7.2
```

##### Getting version from Cargo.toml

```rust
ss rand --manifest
ss rand -m
```

##### Sending a query.

```rust
ss rand -q Rng
ss rand --query Rng
```
