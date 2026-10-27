use crate::{error, outcome::Outcome};
use std::{io, path::PathBuf};

pub fn run(arguments: &[&str]) -> Result<Outcome, error::Error> {
    let path = match arguments.first().copied() {
        None | Some("~") => std::env::home_dir(),
        Some(argument) => match argument.strip_prefix("~/") {
            Some(rest) => std::env::home_dir().map(|home| home.join(rest)),
            None => Some(PathBuf::from(argument)),
        },
    };

    let Some(path) = path else {
        return Ok(Outcome::Continue);
    };

    std::env::set_current_dir(&path).map_err(|e| error::Error::Cd {
        context: format!("cd: {}", path.display()),
        message: match e.kind() {
            io::ErrorKind::NotFound => "No such file or directory",
            io::ErrorKind::NotADirectory => "Not a directory",
            io::ErrorKind::PermissionDenied => "Permission denied",
            _ => "could not change directory",
        },
        source: e,
    })?;

    Ok(Outcome::Continue)
}
