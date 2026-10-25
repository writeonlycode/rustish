use crate::{error::Error, outcome::Outcome};

pub mod echo;
pub mod exit;
pub mod type_command;

pub fn execute(command: &str, arguments: &[&str]) -> Option<Result<Outcome, Error>> {
    match command {
        "type" => Some(type_command::type_command(arguments)),
        "echo" => Some(echo::echo(arguments)),
        "exit" => Some(exit::exit()),
        _ => None,
    }
}
