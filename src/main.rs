use std::{env, process::ExitCode};

use rustish::Shell;

fn main() -> ExitCode {
    let pwd = env::current_dir().unwrap();
    let shell = Shell::new(pwd);

    match shell.run() {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
