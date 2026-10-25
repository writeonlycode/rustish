use crate::{
    commands::Builtin::{Echo, Exit, Type},
    error::Error,
    outcome::Outcome,
};

mod echo;
mod exit;
mod type_builtin;

pub enum Builtin {
    Type,
    Echo,
    Exit,
}

impl Builtin {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "type" => Some(Type),
            "echo" => Some(Echo),
            "exit" => Some(Exit),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Type => "type",
            Echo => "echo",
            Exit => "exit",
        }
    }
}

pub fn execute(command: &str, arguments: &[&str]) -> Option<Result<Outcome, Error>> {
    let command = Builtin::from_name(command)?;

    match command {
        Type => Some(type_builtin::run(arguments)),
        Echo => Some(echo::run(arguments)),
        Exit => Some(exit::run(arguments)),
    }
}
