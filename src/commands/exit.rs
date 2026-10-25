use crate::{error, outcome::Outcome};

pub fn exit() -> Result<Outcome, error::Error> {
    Ok(Outcome::Exit(0))
}
