# aoc24
## Introduction

This is a repository containing solutions for advent of code 2024. <br>
The project is mainly for me to learn the rust programming language, so excuse the sh*tty code.

## Test solutions
`lib.rs` exports solutions defined in `days`. <br>
To test a solution, just edit the line:
```rs
// main.rs
let mut d = D1p2::new();
```
No further edits are needed since all days implement a trait defined in `day.rs`

## Building
This crate uses the `cargo` build system.<br>
```sh
cargo build && cargo run
```

