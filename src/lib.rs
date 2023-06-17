use crate::syntax::AlephTree as at;

pub mod syntax;

pub fn gen_list_expr_sep(ast_list: Vec<Box<at>>, f: fn(at, i64) -> String, sep: &str) -> String {
    format!("{}", ast_list.into_iter().map(|e| f(*e, 0)).collect::<Vec<String>>().join(sep))
}

pub fn gen_list_expr(ast_list: Vec<Box<at>>, f: fn(at, i64) -> String) -> String {
    gen_list_expr_sep(ast_list, f, " ")
}

// indentation for generators
pub fn comp_indent_sep(indent: i64, sep: String) -> String {
    let mut res = "".to_string();
    for _ in 0..indent {
        res.push_str(&sep);
    }
    res
}

// use comp_indent_sep with tab
pub fn comp_indent(indent: i64) -> String {
    comp_indent_sep(indent, String::from("    "))
}
