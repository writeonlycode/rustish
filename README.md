# rustish

`rustish` is a small Unix shell written in Rust. It is being built while working through the [CodeCrafters "Build your own shell" challenge](https://codecrafters.io/challenges/shell).

The goal is a readable, from-scratch shell that can:

- Print a prompt and read commands interactively
- Execute builtins and external programs found on `PATH`
- Tokenize quotes and escapes correctly
- Report errors on stderr and exit with an explicit status

## Features

- **REPL** with a `$ ` prompt, EOF handling, and one error message per failed command.
- **Builtins:**
  - `echo [args...]` — join arguments with a space and print them
  - `pwd` — print the current working directory
  - `cd [dir]` — change directory. With no argument or with `~`, go to the home directory; `~/path` expands relative to the home directory. Errors are reported as clean messages for missing files, non-directories, and permission denied.
  - `type [name...]` — report whether each name is a shell builtin, an executable on `PATH`, or unknown
  - `exit [code]` — exit with an optional `u8` status (default `0`)
- **External commands** are looked up with [`which`](https://crates.io/crates/which) and started with `std::process::Command` using the program name.
- **Quoting and escaping:**
  - Single quotes preserve every character literally, including spaces, backslash, double quote, and `$`.
  - Double quotes preserve spaces and most characters; backslashes escape `"` and `\`.
  - Backslashes escape the next character outside quotes.
  - Quotes can be mixed into a token (`a'b c'd`).
