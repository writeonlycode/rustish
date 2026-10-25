use std::process;

use crate::{
    error::{self, Error::CommandNotFound},
    outcome::Outcome,
};

pub fn run_external_program(name: &str, arguments: &[&str]) -> Result<Outcome, error::Error> {
    let _path = which::which(name).map_err(|_| CommandNotFound(name.to_string()))?;

    let _ = process::Command::new(name)
        .args(arguments)
        .status()
        .map_err(|source| error::Error::Io {
            context: name.to_string(),
            source,
        })?;
    Ok(Outcome::Continue)
}
