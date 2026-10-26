use crate::{error, outcome::Outcome};
use std::{io, path::Path};

pub fn run(arguments: &[&str]) -> Result<Outcome, error::Error> {
    // let s = arguments.join(" ");
    // let mut stdout = io::stdout().lock();

    let Some(argument) = arguments.first() else {
        return Ok(Outcome::Continue);
    };

    let path = Path::new(argument);

    std::env::set_current_dir(path).map_err(|e| error::Error::Cd {
        context: format!("cd: {}", path.display()),
        message: match e.kind() {
            io::ErrorKind::NotFound => "No such file or directory".to_string(),
            _ => e.to_string(),
        },
        source: e,
    })?;

    Ok(Outcome::Continue)
}
