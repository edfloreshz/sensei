<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/edfloreshz/sensei/main/docs/assets/logo.png" width="150" />

  <h1>Sensei (先生)</h1>
</div>

![Rust](https://github.com/edfloreshz/sensei/workflows/Rust/badge.svg?branch=main)

Sensei is a simple command line tool to open documentation for any crate in crates.io. 

## Installation
```
cargo install sensei
```

## Usage 

```
sensei <crate> [OPTIONS] [FLAGS]
```

#### Options

```
-v, --version <version>    Opens documentation for a specific version. 
-q, --query <query>      Specifies query to search documentation. 
```

#### Flags
```
-h, --help      Prints help information
-l, --local    Tries to open local documentation.
``` 


#### Example
```
sensei serde -v 0.8.8 -q Serialize 
```
