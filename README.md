<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/edfloreshz/sensei/main/docs/assets/logo.png" width="150" />

  <h1>Sensei (先生)</h1>
</div>

![Rust](https://github.com/edfloreshz/sensei/workflows/Rust/badge.svg?branch=main)

Sensei is a simple command line tool to open documentation for any crate in crates.io. 

## Installation
```rust 
cargo install sensei
```

## Usage 

```rust 
sensei <crate> [OPTIONS] [FLAGS]
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
sensei rand
```
##### Opening local documentation for a crate.
```rust
sensei rand -l
sensei rand --local
```
##### Specifying a version.
```rust 
sensei rand -v 0.7.2
sensei rand --version 0.7.2
```
#### Getting version from Cargo.toml
```rust 
sensei rand --manifest
sensei rand -m
```
##### Sending a query.
```rust 
sensei rand -q Rng
sensei rand --query Rng
```

