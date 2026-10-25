use crate::{error::Error, outcome::Outcome};

mod echo;
mod exit;
mod pwd;
mod type_builtin;

pub enum Builtin {
    Echo,
    Exit,
    Pwd,
    Type,
}

impl Builtin {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "echo" => Some(Builtin::Echo),
            "exit" => Some(Builtin::Exit),
            "pwd" => Some(Builtin::Pwd),
            "type" => Some(Builtin::Type),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Builtin::Echo => "echo",
            Builtin::Exit => "exit",
            Builtin::Pwd => "pwd",
            Builtin::Type => "type",
        }
    }
}

pub fn execute(command: &str, arguments: &[&str]) -> Option<Result<Outcome, Error>> {
    let command = Builtin::from_name(command)?;

    match command {
        Builtin::Echo => Some(echo::run(arguments)),
        Builtin::Exit => Some(exit::run(arguments)),
        Builtin::Pwd => Some(pwd::run()),
        Builtin::Type => Some(type_builtin::run(arguments)),
    }
}
