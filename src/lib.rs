
use regex::Regex;
use std::collections::HashMap;

pub mod syntax;
pub mod gen;

pub type Token = String;
pub type Rules = HashMap< Token, Vec<RuleRepr>>;

#[derive(PartialEq, Debug, Clone)]
pub struct RuleRepr {
    repr: String,
    node: String,
    args: String
}

#[derive(PartialEq, Clone, Debug)]
enum ParsingState {
    Token,
    Rules,
}


fn line_to_rule(line:&str) -> Option<RuleRepr> {
    let rule = Regex::new(r"\**\| (.*) -> ((([[:upper:]]\w+)(\(.*\)))|(\$\d)).*").unwrap(); //(\w\((\$\w,*)*\))
    return match rule.captures(line) {
        Some(m) =>{
            let r_expr = m.get(1).unwrap().as_str().to_string();
            //check whether a resulting node is named
            match m.get(4) {
                Some(_) => {
                    let r_node = m.get(4).unwrap().as_str().to_string();
                    let r_args = m.get(5).unwrap().as_str().to_string();
                    Some(RuleRepr { repr: r_expr, node: r_node, args: r_args})
                },
                None => {
                    let r_node = "".to_string();
                    let r_args = m.get(2).unwrap().as_str().to_string();
                Some(RuleRepr { repr: r_expr, node: r_node, args: r_args})
                }
            }
        },
        None =>  None
    }
}

fn line_to_token(line: &str) -> Option<String> {
    let token_name = Regex::new(r"^(\w+\**)+:\s*$").unwrap();
    match token_name.captures(line) {
        Some(m) => Some(m.get(1)?.as_str().to_string().clone()),
        _ => None
    }
}


pub fn read_grammar(file_content: String) -> Rules {
    let mut state= ParsingState::Token;

    let end_rule = Regex::new(r"^\s*$").unwrap();

    let mut rules: Rules = HashMap::new();
    let mut current_token: Option<Token> = None;

    for line in file_content.as_str().split("\n") {
        match state.clone() {
            ParsingState::Token => {
                current_token = line_to_token(line);
                state = match current_token.clone() {
                    Some(token) => {
                        rules.insert(token, Vec::new());
                        ParsingState::Rules
                    },
                    _ => ParsingState::Token
                };
            },
            ParsingState::Rules => {
                state = match end_rule.captures(line) {
                    Some(_) => {
                        ParsingState::Token
                    },
                    None => {
                        match line_to_rule(line) {
                            Some(repr) => {
                                rules.entry(current_token.clone().unwrap())
                                    .and_modify(|reprs| {reprs.push(repr)});
                            },
                            None => {}
                        };
                        ParsingState::Rules
                    }
                };
            }
        }
    }
    rules
}


#[cfg(test)]
mod test {
    use crate::{
        line_to_rule,
        line_to_token,
        read_grammar,
        RuleRepr,
        Rules
    };
    use std::fs::File;
    use std::io::Read;

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

    #[test]
    fn test_type_rule() {
        let line = "    | integer -> Integer($1)";
        assert_eq!(
            line_to_rule(line),
            Some(RuleRepr{ repr: "integer".to_string(), node: "Integer".to_string(), args:"($1)".to_string()})
            );
    }

    #[test]
    fn test_expr_rule() {
        let line = "    | expr \"-\" expr -> Sub($1, $2)";
        assert_eq!(
            line_to_rule(line),
            Some(RuleRepr{ repr: "expr \"-\" expr".to_string(), node: "Sub".to_string(), args:"($1, $2)".to_string()})
            );
    }

    #[test]
    fn test_regex_int() {
        let line = "    | r\"-{0,1}[0-9]+\" -> $1";
        assert_eq!(
            line_to_rule(line),
            Some(RuleRepr{ repr: "r\"-{0,1}[0-9]+\"".to_string(), node: "".to_string(), args:"$1".to_string()})
            );
    }

    #[test]
    fn test_on_calc_file(){
        let mut file = File::open("./test/calc/calc.alg").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let res: Rules = read_grammar(contents);
        assert_eq!("", format!("{:?}", res));
    }

    #[test]
    fn test_on_alephg(){
        let mut file = File::open("./test/aleph/aleph.alg").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let res: Rules = read_grammar(contents);
        assert_eq!("", format!("{:?}", res));
    }
}