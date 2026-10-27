use std::io::{self, Write};

use crate::{commands::Builtin, error, outcome::Outcome};

pub fn run(arguments: &[&str]) -> Result<Outcome, error::Error> {
    let mut stdout = io::stdout().lock();

    for name in arguments {
        if let Some(command) = Builtin::from_name(name) {
            writeln!(stdout, "{} is a shell builtin", command.name()).map_err(|source| {
                error::Error::Io {
                    context: "echo".to_string(),
                    source,
                }
            })?;
        } else {
            type_executable(name)?;
        }
    }

    stdout.flush().map_err(|source| error::Error::Io {
        context: "echo".to_string(),
        source,
    })?;

    Ok(Outcome::Continue)
}

fn type_executable(name: &str) -> Result<(), error::Error> {
    let mut stdout = io::stdout().lock();

    if let Ok(path) = which::which(name) {
        writeln!(stdout, "{} is {}", name, path.display()).map_err(|source| error::Error::Io {
            context: "echo".to_string(),
            source,
        })?;
    } else {
        writeln!(stdout, "{}: not found", name).map_err(|source| error::Error::Io {
            context: "echo".to_string(),
            source,
        })?;
    }

    stdout.flush().map_err(|source| error::Error::Io {
        context: "echo".to_string(),
        source,
    })?;

    Ok(())
}
