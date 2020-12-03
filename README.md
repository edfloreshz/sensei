<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/edfloreshz/sensei/main/assets/logo/logo.png" width="150" />

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
sensei <crate> 
```

#### Options

```
-v <version>    Opens documentation for a specific version. 
-s <query>      Specifies query to search documentation. 
```

#### Flags
```
-h, --help      Prints help information
-V, --version   Prints version information
``` 


#### Example
```
 sensei serde -v 0.8.8 -s Serialize 
```
