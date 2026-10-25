use std::io::{self, Write};
use std::path::PathBuf;

use crate::{error, outcome::Outcome};

pub fn run(current_dir: &PathBuf) -> Result<Outcome, error::Error> {
    let mut stdout = io::stdout().lock();

    writeln!(stdout, "{}", current_dir.display()).map_err(|e| crate::Error::Io {
        context: "pwd".to_string(),
        source: e,
    })?;

    Ok(Outcome::Continue)
}
