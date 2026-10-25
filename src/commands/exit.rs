use crate::{error, outcome::Outcome};

pub fn run(arguments: &[&str]) -> Result<Outcome, error::Error> {
    match arguments.first() {
        Some(exit_code) => {
            let exit_code: u8 = exit_code
                .parse()
                .map_err(|e| error::Error::InvalidExitCode {
                    context: "exit".to_string(),
                    source: e,
                })?;

            Ok(Outcome::Exit(exit_code))
        }
        None => Ok(Outcome::Exit(0)),
    }
}
