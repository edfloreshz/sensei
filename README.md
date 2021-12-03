<div align="center">
  <br>
  <img src="https://github.com/edfloreshz/sensei/blob/main/docs/assets/logo.png?raw=true" width="150" />
  <h1>Sensei (先生)</h1>
  <a href="https://github.com/edfloreshz/sensei/actions/workflows/rust.yml">
    <img src="https://img.shields.io/github/workflow/status/edfloreshz/sensei/Rust?logo=GitHub" alt="build"/>
  </a>
  <a href="https://crates.io/crates/sensei">
    <img src="https://img.shields.io/crates/v/sensei?label=Sensei" alt="crate"/>
  </a>
   <a href="https://crates.io/crates/sensei">
    <img src="https://img.shields.io/crates/d/sensei" alt="downloads"/>
  </a>
  <a href="https://t.me/sensei_rs">
    <img src="https://img.shields.io/static/v1?label=chat&message=Telegram&color=blue&logo=telegram" alt="chat on telegram"/>
  </a>
  <a href="https://aur.archlinux.org/packages/sensei/">
    <img src="https://img.shields.io/aur/version/sensei" alt="sensei"/>
  </a>
</div>

Sensei is a simple command line tool to open documentation for any crate in crates.io

## Installation

#### Cargo

```shell
cargo install sensei
```

#### Arch Linux

```rust
paru -S sensei
```

## Usage

```rust
sns <crate> [OPTIONS] [FLAGS]
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
sns rand
```

##### Opening local documentation for a crate.

```rust
sns rand -l
sns rand --local
```

##### Specifying a version.

```rust
sns rand -v 0.7.2
sns rand --version 0.7.2
```

##### Getting version from Cargo.toml

```rust
sns rand --manifest
sns rand -m
```

##### Sending a query.

```rust
sns rand -q Rng
sns rand --query Rng
```
