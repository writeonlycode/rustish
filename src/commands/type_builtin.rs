use crate::{commands::Builtin, error, outcome::Outcome};

pub fn run(arguments: &[&str]) -> Result<Outcome, error::Error> {
    for name in arguments {
        if let Some(command) = Builtin::from_name(name) {
            println!("{} is a shell builtin", command.name());
        } else {
            type_executable(name);
        }
    }

    Ok(Outcome::Continue)
}

fn type_executable(name: &str) {
    if let Ok(path) = which::which(name) {
        println!("{name} is {}", path.display());
    } else {
        println!("{name}: not found")
    }
}
