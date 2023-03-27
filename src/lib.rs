
use regex::Regex;

fn line_to_token(line: &str) -> Option<String> {
    let token_name = Regex::new(r"^(\w+\**)+:\s*$").unwrap();
    match token_name.captures(line) {
        Some(m) => Some(m.get(1)?.as_str().to_string().clone()),
        _ => None
    }
}



#[cfg(test)]
mod test {
    use crate::{
        line_to_token,
    };

    #[test]
    fn test_token() {
        let line = "expr:";
        assert_eq!(
            line_to_token(line),
            Some("expr".to_string())
            );
    }
    #[test]
    fn test_token_none() {
        let line = "      ";
        assert_eq!(
            line_to_token(line),
            None
            );
    }
}
