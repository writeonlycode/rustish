use std::io::{self, BufRead, Write};

use crate::commands;
use crate::error;
use crate::executor;
use crate::outcome;
use crate::parser;

pub fn run() -> Result<u8, error::Error> {
    loop {
        print_prompt().map_err(|source| error::Error::Io {
            context: "print_prompt".to_string(),
            source,
        })?;

        let Some(line) = read_line().map_err(|source| error::Error::Io {
            context: "read_line".to_string(),
            source,
        })?
        else {
            break Ok(0);
        };

        let Some((command, arguments)) = parser::parse(&line) else {
            continue;
        };

        match execute_line(command, &arguments) {
            Ok(outcome::Outcome::Continue) => {}
            Ok(outcome::Outcome::Exit(code)) => break Ok(code),
            Err(error) => report_command_error(error),
        }
    }
}

fn print_prompt() -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(b"$ ")?;
    stdout.flush()
}

fn read_line() -> io::Result<Option<String>> {
    let mut line = String::new();
    let bytes_read = io::stdin().lock().read_line(&mut line)?;

    // Handles EOF
    if bytes_read == 0 {
        return Ok(None);
    }

    Ok(Some(line))
}

fn execute_line(command: &str, arguments: &[&str]) -> Result<outcome::Outcome, error::Error> {
    match commands::execute(command, arguments) {
        Some(result) => result,
        None => executor::run_external_program(command, arguments),
    }
}

fn report_command_error(e: error::Error) {
    eprintln!("{e}");
}
