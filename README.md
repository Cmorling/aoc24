# aoc24
## Introduction

This is a repository containing solutions for advent of code 2024. <br>
The project is mainly for me to learn the rust programming language, so excuse the sh*tty code.<br>

## Usage
```
Solver for Advent of code 2024 written in rust (author cBang)

Usage: aoc24 [OPTIONS] -d <DAY> -p <PART>

Options:
  -d <DAY>       Day to solve
  -p <PART>      Part to solve
  -b             Benchmark solution with criterion
  -h, --help     Print help
  -V, --version  Print version
```

## Building
This crate uses the `cargo` build system.<br>
```sh
cargo build
```

## Adding solutions
Add your solution in `src/days/` Make sure the struct is named `Dx` where `x` is that day. <Br>
Make sure the struct implements the `Day` trait. <br>
Export your solution in `src/lib.rs`
