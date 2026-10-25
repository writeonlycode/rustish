pub fn parse(line: &str) -> Option<(&str, Vec<&str>)> {
    let mut tokens = line.split_whitespace();
    let command = tokens.next()?;
    Some((command, tokens.collect()))
}
