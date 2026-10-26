enum State {
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
    UnquotedBackslash,
    DoubleQuotedBackslash,
}

pub fn parse(line: &str) -> Option<(String, Vec<String>)> {
    let mut state = State::Unquoted;
    let mut current = String::new();
    let mut tokens = Vec::new();

    let mut characters = line.chars().peekable();

    while let Some(c) = characters.next() {
        match state {
            // Unquoted States
            State::Unquoted if c.is_single_quote() => {
                state = State::SingleQuoted;
            }
            State::Unquoted if c.is_double_quote() => {
                state = State::DoubleQuoted;
            }
            State::Unquoted if c.is_backslash() => {
                state = State::UnquotedBackslash;
            }
            State::Unquoted if c.is_whitespace() => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            State::Unquoted => {
                current.push(c);
            }

            // Single Quote States
            State::SingleQuoted if c.is_single_quote() => {
                state = State::Unquoted;
            }
            State::SingleQuoted => {
                current.push(c);
            }

            // Double Quote States
            State::DoubleQuoted if c.is_double_quote() => {
                state = State::Unquoted;
            }
            State::DoubleQuoted if c.is_backslash() => match characters.peek() {
                Some(n) if n.is_backslash() || n.is_double_quote() => {
                    state = State::DoubleQuotedBackslash;
                }
                _ => {
                    current.push(c);
                }
            },
            State::DoubleQuoted => {
                current.push(c);
            }

            // Backslash States
            State::UnquotedBackslash => {
                current.push(c);
                state = State::Unquoted;
            }
            State::DoubleQuotedBackslash => {
                current.push(c);
                state = State::DoubleQuoted;
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
    fn is_backslash(&self) -> bool;
}

impl CharExt for char {
    fn is_single_quote(&self) -> bool {
        *self == '\''
    }

    fn is_double_quote(&self) -> bool {
        *self == '\"'
    }

    fn is_backslash(&self) -> bool {
        *self == '\\'
    }
}
