use crate::{error, outcome::Outcome};

pub fn type_command(arguments: &[&str]) -> Result<Outcome, error::Error> {
    match arguments.into_iter().next() {
        Some(&"type") => {
            println!("type is a shell builtin");
            Ok(Outcome::Continue)
        }
        Some(&"echo") => {
            println!("echo is a shell builtin");
            Ok(Outcome::Continue)
        }
        Some(&"exit") => {
            println!("exit is a shell builtin");
            Ok(Outcome::Continue)
        }
        Some(name) => type_executable(name),
        None => {
            println!("type must have 1 argument");
            Ok(Outcome::Continue)
        }
    }
}

fn type_executable(name: &str) -> Result<Outcome, error::Error> {
    if let Ok(path) = which::which(name) {
        println!("{name} is {}", path.display());
    } else {
        println!("{name}: not found")
    }

    Ok(Outcome::Continue)
}
