# rust_one_time_pad
One Time Pad Implementation in Rust.
**Do not use this library for anything other than testing purposes.**

## One-time Pad Description
[Based off the one-time pad description in 0-vername.pdf](https://github.com/JeffResc/rust_one_time_pad/blob/main/0-vernam.pdf)

## Features
* Encrypt files
* Decrypt files

## Known limitations
* Easy to break as random bit b only has two options, 0 or 1
* Limited to files with strings

## Dependencies
* [clap](https://crates.io/crates/clap)

## Installation
```bash
cargo install --git https://github.com/JeffResc/rust_one_time_pad
```

## Usage

For Windows:
```bash
rust_one_time_pad <SUBCOMMAND>
```

For Linux:
```bash
./rust_one_time_pad <SUBCOMMAND>
```

### Main
```
One Time Pad Implementation in Rust.

USAGE:
    rust_one_time_pad <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    decrypt    Decrypts a file
    encrypt    Encrypts a file
    help       Print this message or the help of the given subcommand(s)
```

### Decrypt
```
Decrypts a file

USAGE:
    rust_one_time_pad decrypt <RANDOM_BIT> <INPUT_FILE> <OUTPUT_FILE>

ARGS:
    <RANDOM_BIT>     Random bit r
    <INPUT_FILE>     File to decrypt
    <OUTPUT_FILE>    Decrypted file

OPTIONS:
    -h, --help    Print help information
```

### Encrypt
```
Encrypts a file

USAGE:
    rust_one_time_pad encrypt <RANDOM_BIT> <INPUT_FILE> <OUTPUT_FILE>

ARGS:
    <RANDOM_BIT>     Random bit r
    <INPUT_FILE>     File to encrypt
    <OUTPUT_FILE>    Encrypted file

OPTIONS:
    -h, --help    Print help information
```
