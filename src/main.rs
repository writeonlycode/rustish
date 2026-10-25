use std::process::ExitCode;

fn main() -> ExitCode {
    match rustish::run() {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
