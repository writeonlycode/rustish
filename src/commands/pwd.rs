use std::env::current_dir;
use std::io::{self, Write};

use crate::{error, outcome::Outcome};

pub fn run() -> Result<Outcome, error::Error> {
    let mut stdout = io::stdout().lock();

    let current_dir = current_dir().map_err(|e| error::Error::Io {
        context: "pwd".to_string(),
        source: e,
    })?;

    writeln!(stdout, "{}", current_dir.display()).map_err(|e| error::Error::Io {
        context: "pwd".to_string(),
        source: e,
    })?;

    Ok(Outcome::Continue)
}
