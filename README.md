# LM Compiler Rust

Simple compiler written in Rust using:

- [Rustemo](https://github.com/igordejanovic/rustemo) as the parser generator
- [rflex](https://github.com/pfnet/rflex) as the lexer generator

## Prerequisites

Install [Rust](https://rustup.rs/)

## Features

### v1.0.0

|Feature|Status|File/s|
|-------|------|------|
|Lexer|✅|[rflex lexer file](./src/lex.l)|
|Grammar|✅|[rustemo grammar file](./src/rustemo.grammar)|
|Symbol table|✅|Generate running the executable|
|Tests|✅|Run using `cargo test`|

### v2.0.0
|Feature|Status|File/s|
|-------|------|------|
|Intermediate code(Tree)|🏗️|-|
|Tests|🏗️|-|

### v3.0.0
|Feature|Status|File/s|
|-------|------|------|
|Assembler output|🏗️|-|
|Linking script|🏗️|-|

## Compiling

1. Compile in debug mode
```Shell
cargo build
```

2. Compile in release mode
```Shell
cargo build --release
```

## Running the compiler

1. Run in debug mode
```Shell
cargo run
```

2. Run in release mode
```Shell
cargo run --release
```

The executable can accept an input file as a parmeter and will by default run the file located at './inputs/test.txt'.

## Running the tests

The integration test runs the compiler on all the files located at `examples` folder. Currently it only asserts that the compiler can run without errors on all the files, it does not check that the compilation output is correct.

```
cargo test
```

## Generated files

When running the compiler it will output 3 files to the path of the source file

- \<source-file-name\>.parser
- \<source-file-name\>.lexer
- \<source-file-name\>.symbol_table

## Cli reference

```
Usage: lm-compiler [INPUT]

Arguments:
  [INPUT]  Path to the source code file [default: ./Inputs/test.txt]

Options:
  -h, --help     Print help
  -V, --version  Print version
```
