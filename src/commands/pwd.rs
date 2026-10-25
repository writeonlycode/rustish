use std::env::current_dir;
use std::io::{self, Write};

use crate::{error, outcome::Outcome};

pub fn run() -> Result<Outcome, error::Error> {
    let mut stdout = io::stdout().lock();

    let dir = current_dir().map_err(|e| error::Error::Io {
        context: "pwd".to_string(),
        source: e,
    })?;

    writeln!(stdout, "{}", dir.display()).map_err(|e| error::Error::Io {
        context: "pwd".to_string(),
        source: e,
    })?;

    stdout.flush().map_err(|source| error::Error::Io {
        context: "echo".to_string(),
        source,
    })?;

    Ok(Outcome::Continue)
}
