# RustBrain
## Description

RustBrain is BrainFuck like language thats written in rust.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)

## Usage

- building *optional*
    0. first make sure to install `rustup/cargo`
    1. clone this repo: `git clone https://github.com/LuznyKamus1/RustBrain.git`
    2. cd into the downloaded repo: `cd RustBrain`
    3. build the project: `cargo build`

- Running

0. there are two options for running the project:

1. first one is only available if you built the project from source:
    1. `cd path/to/RustBrain`
    2. `cargo run -- /path/to/file.rbrain`
2. second one is available in every scenario:
    1. `cd path/to/binary`
    2. `./binary /path/to/file.rbrain`

## Features

Syntax:
	`>` - move pointer by 1
	`<` - move pointer by -1
	`+` - add 1 to current position
	`-` - substract 1 from current position
	`!` - print value from current position
