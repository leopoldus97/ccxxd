# Coding Challenge: `Xxd`

This application was created as part of a coding challenge. The challenge details can be found [here](https://codingchallenges.fyi/challenges/challenge-xxd).

The following resources were used for implementing and understanding the functionality of the `xxd` command:
- [Wikipedia - Hex dump](https://en.wikipedia.org/wiki/Hex_dump)
- [Manpages - Xxd Manual](https://manpages.org/xxd)

## Description

A simple xxd-like ("hex dump") cli tool created in Rust. This implementation mimics some, but not all of the functionalities of the original `xxd` command.

## How to Run

To run the ccxxd command line tool, follow these steps:
1. Clone the repository.
2. Build the applikcation using `cargo build --release`. That will generate the executable in the `target/release` directory called `ccxxd`.
3. Run the application.

## Arguments

You can run the application with the following arguments:
- `ccxxd -c <input> <output>` to specify the number of bytes per line.
- `ccxxd -e <input> <output>` to set the little-endian byte order.
- `ccxxd -g <input> <output>` to specify the number of bytes per group.
- `ccxxd -l <input> <output>` to specify the number of bytes to write.
- `ccxxd -r <input> <output>` to reverse the process, reading a hexdump and writing the original file.
- `ccxxd -s <input> <output>` to skip a number of bytes.

## Notes

The application accepts file paths as the input and output arguments. The input file is the file to be read and the output file is the file to be written to. If the input file is not specified then the application reads from stdin. If the output file is not specified then the application writes to stdout.
