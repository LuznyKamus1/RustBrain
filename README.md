# RustBrain
## Description

RustBrain is BrainFuck like language thats written in rust.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)

## Usage

- Building
    1. first make sure to install `rustup/cargo`
    2. clone this repo: `git clone https://github.com/LuznyKamus1/RustBrain.git`
    3. cd into the downloaded repo: `cd RustBrain`
    4. build the project: `cargo build`

- Downloading the binary
    1. you can download the binary from the realeses section in github if you do not want to buid the package yourself but it is not guaranteed to work on every machine

- Running

0. there are two options for running the project:

1. first one is only available if you built the project from source:
    1. `cd path/to/RustBrain`
    2. `cargo run -- /path/to/file.rbrain`
2. second one is available in every scenario:
    1. `cd path/to/binary`
    2. `./binary /path/to/file.rbrain`

## Syntax
1. `>` - move pointer by 1
2. `<` - move pointer by -1
3. `+` - add 1 to current position
4. `-` - substract 1 from current position
5. `!` - print value from current position
6. `[` - jumps to the next `]` if current possition == 0
5. `]` - jumps to the previous `[` if current possition != 0
