use strum_macros::EnumString;
use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Debug, Serialize, Deserialize, Clone, EnumString)]
#[serde(tag="type")]
pub enum AlephTree {
    #[default]
    Unit,
    Break,
    Continue,
    Ellipsis,
    #[serde(alias="Integer")]
    Int{value:String},
    Float{value:String},
    Bool{value:String},
    String{value:String},
    Bytes{elems: Vec<u8>},
    Complex{real: String, imag: String},
    Tuple {
        elems: Vec<Box<AlephTree>>
    },
    Array {
        elems: Vec<Box<AlephTree>>
    },
    Neg{
        expr:Box<AlephTree>,
    },
    Not{
        #[serde(alias="boolExpr")]
        bool_expr:Box<AlephTree>,
    },
    And{
        #[serde(alias="boolExpr1")]
        bool_expr1:Box<AlephTree>,
        #[serde(alias="boolExpr2")]
        bool_expr2:Box<AlephTree>
    },
    Or{
        #[serde(alias="boolExpr1")]
        bool_expr1:Box<AlephTree>,
        #[serde(alias="boolExpr2")]
        bool_expr2:Box<AlephTree>
    },
    Add{
        #[serde(alias="numberExpr1")]
        number_expr1:Box<AlephTree>,
        #[serde(alias="numberExpr2")]
        number_expr2:Box<AlephTree>
    },
    Sub{
        #[serde(alias="numberExpr1")]
        number_expr1:Box<AlephTree>,
        #[serde(alias="numberExpr2")]
        number_expr2:Box<AlephTree>
    },
    Mul{
        #[serde(alias="numberExpr1")]
        number_expr1:Box<AlephTree>,
        #[serde(alias="numberExpr2")]
        number_expr2:Box<AlephTree>
    },
    Div{
        #[serde(alias="numberExpr1")]
        number_expr1:Box<AlephTree>,
        #[serde(alias="numberExpr2")]
        number_expr2:Box<AlephTree>
    },
    Eq{
        expr1: Box<AlephTree>,
        expr2: Box<AlephTree>,
    },
    LE{
        expr1: Box<AlephTree>,
        expr2: Box<AlephTree>,
    },
    In{
        expr1: Box<AlephTree>,
        expr2: Box<AlephTree>,
    },
    If{
        condition: Box<AlephTree>,
        then: Box<AlephTree>,
        #[serde(alias="else")]
        els: Box<AlephTree>
    },
    While{
        #[serde(alias="initExpr")]
        init_expr: Box<AlephTree>,
        condition: Box<AlephTree>,
        #[serde(alias="loopExpr")]
        loop_expr: Box<AlephTree>,
        #[serde(alias="postExpr")]
        post_expr: Box<AlephTree>,
    },
    Let{
        var: String,
        #[serde(alias="isPointer")]
        is_pointer: String,
        value: Box<AlephTree>,
        expr: Box<AlephTree>
    },
    LetRec{
        name: String,
        args: Vec<Box<AlephTree>>,
        body: Box<AlephTree>
    },
    Get{
        #[serde(alias="arrayName")]
        array_name: String,
        elem: Box<AlephTree>
    },
    Put{
        #[serde(alias="arrayName")]
        array_name: String,
        elem: Box<AlephTree>,
        value: Box<AlephTree>,
        insert: String
    },
    Remove{
        #[serde(alias="arrayName")]
        array_name: String,
        elem: Box<AlephTree>,
        #[serde(alias="isValue")]
        is_value: String,
    },
    Length{
        var:String
    },
    Match{
        expr: Box<AlephTree>,
        #[serde(alias="caseList")]
        case_list: Vec<Box<AlephTree>>
    },
    MatchLine {
        condition: Box<AlephTree>,
        #[serde(alias="caseExpr")]
        case_expr: Box<AlephTree>
    },
    Var{
        var: String,
        #[serde(alias="isPointer")]
        is_pointer: String,
    },
    App{
        #[serde(alias="objectName")]
        object_name: String,
        fun: Box<AlephTree>,
        #[serde(alias="paramList")]
        param_list: Vec<Box<AlephTree>>
    },
    Stmts {
        expr1: Box<AlephTree>,
        expr2: Box<AlephTree>
    },
    #[serde(alias="Import")]
    Iprt{
        name: String 
    },
    #[serde(alias="Class")]
    Clss{
        name: String,
        #[serde(alias="attributList")]
        attribute_list: Vec<String>,
        body: Box<AlephTree>
    },
    #[serde(alias="Return")]
    Return{
        value: Box<AlephTree>
    },
    #[serde(alias="Comment")]
    Comment{
        value: String 
    },
    #[serde(alias="CommentMulti")]
    CommentMulti{
        value: String 
    },
    Assert {
        condition: Box<AlephTree>,
        message: Box<AlephTree>
    }
}

impl FromIterator<AlephTree> for Vec<Box<AlephTree>> {
    fn from_iter<I: IntoIterator<Item=AlephTree>>(iter : I) -> Self {
        let mut result: Vec<Box<AlephTree>> = Vec::new();
        for node in iter {
            result.push(Box::new(node));
        }
        result
    }
}
