use std::io::{self, Write};

use crate::{error, outcome::Outcome};

pub fn echo(arguments: &[&str]) -> Result<Outcome, error::Error> {
    let s = arguments.join(" ");
    println!("{s}");
    io::stdout().flush().map_err(|source| error::Error::Io {
        context: "echo".to_string(),
        source,
    })?;

    Ok(Outcome::Continue)
}
