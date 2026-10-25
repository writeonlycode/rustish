use std::io::{self, Write};

use crate::{error, outcome::Outcome};

pub fn run(arguments: &[&str]) -> Result<Outcome, error::Error> {
    let s = arguments.join(" ");
    let mut stdout = io::stdout().lock();

    writeln!(stdout, "{s}").map_err(|source| error::Error::Io {
        context: "echo".to_string(),
        source,
    })?;

    stdout.flush().map_err(|source| error::Error::Io {
        context: "echo".to_string(),
        source,
    })?;

    Ok(Outcome::Continue)
}
