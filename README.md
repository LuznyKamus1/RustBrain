# RustBrain
## Description

RustBrain is BrainFuck like language thats written in rust.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)
- [Contributing](#contributing)
- [License](#license)

## Installation

To install and run the project, follow these steps:

0. if you havent already, install `cargo` and `git`
1. Clone the repository: `git clone https://github.com/LuznyKamus1/RustBrain.git`
2. Navigate to the project directory: `cd RustBrain`
4. Build the project: `cargo build --release`
5. Start the application: `cargo run --release ./filename.rbrain`

## Features

Syntax:
	`>` - move pointer by 1
	`<` - move pointer by -1
	`+` - add 1 to current position
	`-` - substract 1 from current position
	`!` - print value from current position
