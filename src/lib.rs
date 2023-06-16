use crate::syntax::AlephTree as at;

pub mod syntax;
pub mod gen;

/**
* this trait should be implemented by all parser
*/
pub trait Parser {
    fn parse(&self, source: String) -> at;
}
