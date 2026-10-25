use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use crate::commands;
use crate::error;
use crate::executor;
use crate::outcome;
use crate::parser;

pub struct Shell {
    current_dir: PathBuf,
}

impl Shell {
    pub fn new(current_dir: PathBuf) -> Self {
        Shell { current_dir }
    }

    pub fn run(&self) -> Result<u8, error::Error> {
        loop {
            Shell::print_prompt().map_err(|source| error::Error::Io {
                context: "print_prompt".to_string(),
                source,
            })?;

            let Some(line) = Shell::read_line().map_err(|source| error::Error::Io {
                context: "read_line".to_string(),
                source,
            })?
            else {
                break Ok(0);
            };

            let Some((command, arguments)) = parser::parse(&line) else {
                continue;
            };

            match self.execute_line(command, &arguments) {
                Ok(outcome::Outcome::Continue) => {}
                Ok(outcome::Outcome::Exit(code)) => break Ok(code),
                Err(error) => Shell::report_command_error(error),
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

    fn execute_line(
        &self,
        command: &str,
        arguments: &[&str],
    ) -> Result<outcome::Outcome, error::Error> {
        match commands::execute(command, &arguments, &self.current_dir) {
            Some(result) => result,
            None => executor::run_external_program(command, &arguments),
        }
    }

    fn report_command_error(e: error::Error) {
        eprintln!("{e}");
    }
}
