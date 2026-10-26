enum State {
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
}

pub fn parse(line: &str) -> Option<(String, Vec<String>)> {
    let mut state = State::Unquoted;
    let mut current = String::new();
    let mut tokens = Vec::new();

    for c in line.chars() {
        match state {
            State::Unquoted if c.is_single_quote() => {
                state = State::SingleQuoted;
            }
            State::Unquoted if c.is_double_quote() => {
                state = State::DoubleQuoted;
            }
            State::Unquoted if c.is_whitespace() => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            State::Unquoted => {
                current.push(c);
            }

            State::SingleQuoted if c.is_single_quote() => {
                state = State::Unquoted;
            }
            State::SingleQuoted => {
                current.push(c);
            }

            State::DoubleQuoted if c.is_double_quote() => {
                state = State::Unquoted;
            }
            State::DoubleQuoted => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        tokens.push(std::mem::take(&mut current));
    }

    let mut tokens = tokens.into_iter();

    let command = tokens.next()?;

    Some((command, tokens.collect()))
}

trait CharExt {
    fn is_single_quote(&self) -> bool;
    fn is_double_quote(&self) -> bool;
}

impl CharExt for char {
    fn is_single_quote(&self) -> bool {
        *self == '\''
    }

    fn is_double_quote(&self) -> bool {
        *self == '\"'
    }
}
