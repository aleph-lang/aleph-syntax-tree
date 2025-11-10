use std::fmt;
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
    Ident{value:String},
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
    },

    CobolProgram {
        #[serde(alias="programId")]
        program_id: String,
        #[serde(alias="environmentDiv")]
        environment_div: Option<Box<AlephTree>>,
        #[serde(alias="dataDiv")]
        data_div: Option<Box<AlephTree>>,
        #[serde(alias="procedureDiv")]
        procedure_div: Box<AlephTree>
    },

    EnvironmentDivision {
        #[serde(alias="configSection")]
        config_section: Option<Box<AlephTree>>,
        #[serde(alias="ioControlSection")]
        io_control_section: Option<Box<AlephTree>>
    },

    DataDivision {
        #[serde(alias="fileSection")]
        file_section: Option<Box<AlephTree>>,
        #[serde(alias="workingStorageSection")]
        working_storage_section: Option<Box<AlephTree>>,
        #[serde(alias="linkageSection")]
        linkage_section: Option<Box<AlephTree>>
    },

    ProcedureDivision {
        #[serde(alias="usingClause")]
        using_clause: Option<Vec<String>>,
        statements: Vec<Box<AlephTree>>
    },

    PicClause {
        #[serde(alias="dataName")]
        data_name: String,
        #[serde(alias="levelNumber")]
        level_number: String,
        picture: String,
        #[serde(alias="initialValue")]
        initial_value: Option<Box<AlephTree>>,
        #[serde(alias="occursClause")]
        occurs_clause: Option<String>,
        usage: Option<String>
    },

    GroupItem {
        #[serde(alias="dataName")]
        data_name: String,
        #[serde(alias="levelNumber")]
        level_number: String,
        #[serde(alias="subItems")]
        sub_items: Vec<Box<AlephTree>>
    },

    Redefines {
        #[serde(alias="dataName")]
        data_name: String,
        #[serde(alias="levelNumber")]
        level_number: String,
        #[serde(alias="redefinedItem")]
        redefined_item: String,
        picture: Option<String>
    },

    FileDescription {
        #[serde(alias="fileName")]
        file_name: String,
        #[serde(alias="recordDescription")]
        record_description: Vec<Box<AlephTree>>,
        #[serde(alias="blockContains")]
        block_contains: Option<String>,
        #[serde(alias="recordContains")]
        record_contains: Option<String>
    },

    SelectStatement {
        #[serde(alias="fileName")]
        file_name: String,
        #[serde(alias="assignTo")]
        assign_to: String,
        #[serde(alias="accessMode")]
        access_mode: Option<String>,
        #[serde(alias="organizationMode")]
        organization_mode: Option<String>
    },

    Move {
        source: Box<AlephTree>,
        #[serde(alias="targetList")]
        target_list: Vec<Box<AlephTree>>
    },

    Compute {
        target: Box<AlephTree>,
        expression: Box<AlephTree>,
        #[serde(alias="onSizeError")]
        on_size_error: Option<Box<AlephTree>>
    },

    Perform {
        #[serde(alias="targetParagraph")]
        target_paragraph: Option<String>,
        #[serde(alias="fromParagraph")]
        from_paragraph: Option<String>,
        #[serde(alias="throughParagraph")]
        through_paragraph: Option<String>,
        #[serde(alias="timesClause")]
        times_clause: Option<Box<AlephTree>>,
        #[serde(alias="untilClause")]
        until_clause: Option<Box<AlephTree>>,
        #[serde(alias="varyingClause")]
        varying_clause: Option<Box<AlephTree>>,
        #[serde(alias="inlineStatements")]
        inline_statements: Option<Vec<Box<AlephTree>>>
    },

    Accept {
        target: Box<AlephTree>,
        #[serde(alias="fromDevice")]
        from_device: Option<String>
    },

    Display {
        #[serde(alias="itemList")]
        item_list: Vec<Box<AlephTree>>,
        #[serde(alias="uponDevice")]
        upon_device: Option<String>
    },

    Open {
        mode: String,
        #[serde(alias="fileList")]
        file_list: Vec<String>
    },

    Close {
        #[serde(alias="fileList")]
        file_list: Vec<String>
    },

    Read {
        #[serde(alias="fileName")]
        file_name: String,
        #[serde(alias="intoClause")]
        into_clause: Option<Box<AlephTree>>,
        #[serde(alias="keyClause")]
        key_clause: Option<Box<AlephTree>>,
        #[serde(alias="atEndClause")]
        at_end_clause: Option<Box<AlephTree>>,
        #[serde(alias="notAtEndClause")]
        not_at_end_clause: Option<Box<AlephTree>>
    },

    Write {
        #[serde(alias="recordName")]
        record_name: String,
        #[serde(alias="fromClause")]
        from_clause: Option<Box<AlephTree>>,
        #[serde(alias="advancingClause")]
        advancing_clause: Option<String>
    },

    GoTo {
        #[serde(alias="targetParagraph")]
        target_paragraph: String,
        #[serde(alias="dependingOn")]
        depending_on: Option<Box<AlephTree>>
    },

    Stop {
        #[serde(alias="stopType")]
        stop_type: String
    },

    Exit,

    Paragraph {
        name: String,
        statements: Vec<Box<AlephTree>>
    },

    Section {
        name: String,
        paragraphs: Vec<Box<AlephTree>>
    },

    Evaluate {
        #[serde(alias="selectionSubject")]
        selection_subject: Box<AlephTree>,
        #[serde(alias="whenClauses")]
        when_clauses: Vec<Box<AlephTree>>,
        #[serde(alias="whenOther")]
        when_other: Option<Box<AlephTree>>
    },

    WhenClause {
        #[serde(alias="selectionObject")]
        selection_object: Box<AlephTree>,
        statements: Vec<Box<AlephTree>>
    },

    Inspect {
        #[serde(alias="inspectingItem")]
        inspecting_item: Box<AlephTree>,
        #[serde(alias="tallyingClause")]
        tallying_clause: Option<Box<AlephTree>>,
        #[serde(alias="replacingClause")]
        replacing_clause: Option<Box<AlephTree>>
    },

    StringStmt {
        #[serde(alias="sourceItems")]
        source_items: Vec<Box<AlephTree>>,
        #[serde(alias="delimitedBy")]
        delimited_by: Box<AlephTree>,
        #[serde(alias="intoItem")]
        into_item: Box<AlephTree>,
        #[serde(alias="withPointer")]
        with_pointer: Option<Box<AlephTree>>,
        #[serde(alias="onOverflow")]
        on_overflow: Option<Box<AlephTree>>
    },

    Unstring {
        #[serde(alias="sourceItem")]
        source_item: Box<AlephTree>,
        #[serde(alias="delimitedBy")]
        delimited_by: Box<AlephTree>,
        #[serde(alias="intoItems")]
        into_items: Vec<Box<AlephTree>>,
        #[serde(alias="withPointer")]
        with_pointer: Option<Box<AlephTree>>,
        #[serde(alias="onOverflow")]
        on_overflow: Option<Box<AlephTree>>
    },

    OnSizeError {
        statements: Vec<Box<AlephTree>>
    },

    NotOnSizeError {
        statements: Vec<Box<AlephTree>>
    },

    Call {
        #[serde(alias="programName")]
        program_name: Box<AlephTree>,
        #[serde(alias="usingParameters")]
        using_parameters: Option<Vec<Box<AlephTree>>>,
        #[serde(alias="givingParameter")]
        giving_parameter: Option<Box<AlephTree>>,
        #[serde(alias="onException")]
        on_exception: Option<Box<AlephTree>>
    },

    QualifiedName {
        #[serde(alias="dataName")]
        data_name: String,
        #[serde(alias="qualifierList")]
        qualifier_list: Vec<String>
    },

    Subscript {
        #[serde(alias="dataName")]
        data_name: String,
        #[serde(alias="subscriptList")]
        subscript_list: Vec<Box<AlephTree>>
    },

    Figurative {
        #[serde(alias="figurativeType")]
        figurative_type: String
    },

    ClassCondition {
        #[serde(alias="dataItem")]
        data_item: Box<AlephTree>,
        #[serde(alias="className")]
        class_name: String
    },

    SignCondition {
        #[serde(alias="dataItem")]
        data_item: Box<AlephTree>,
        sign: String
    },

    OccursClause {
        #[serde(alias="minOccurs")]
        min_occurs: Option<String>,
        #[serde(alias="maxOccurs")]
        max_occurs: String,
        #[serde(alias="dependingOn")]
        depending_on: Option<String>,
        #[serde(alias="indexedBy")]
        indexed_by: Option<Vec<String>>
    },

    HexLiteral {
        value: String
    },

    UsageClause {
        #[serde(alias="usageType")]
        usage_type: String
    },

    // === Forth-specific nodes ===

    // Forth word definition
    ForthDef {
        name: String,
        body: Vec<Box<AlephTree>>,
        #[serde(alias="isImmediate")]
        is_immediate: bool,
    },

    // Forth constant
    ForthConst {
        name: String,
        value: Box<AlephTree>,
    },

    // Forth variable
    ForthVar {
        name: String,
    },

    // Forth CREATE...DOES>
    ForthCreate {
        name: String,
        allot_size: Option<Box<AlephTree>>,
        does_body: Option<Vec<Box<AlephTree>>>,
    },

    // Stack operations
    ForthDup,      // DUP ( n -- n n )
    ForthDrop,     // DROP ( n -- )
    ForthSwap,     // SWAP ( n1 n2 -- n2 n1 )
    ForthOver,     // OVER ( n1 n2 -- n1 n2 n1 )
    ForthRot,      // ROT ( n1 n2 n3 -- n2 n3 n1 )
    ForthMinusRot, // -ROT ( n1 n2 n3 -- n3 n1 n2 )
    ForthNip,      // NIP ( n1 n2 -- n2 )
    ForthTuck,     // TUCK ( n1 n2 -- n2 n1 n2 )
    ForthPick {    // PICK ( ... n -- ... x )
        depth: Box<AlephTree>,
    },
    ForthRoll {    // ROLL ( ... n -- ... )
        depth: Box<AlephTree>,
    },

    // Double stack operations
    ForthTwoDup,   // 2DUP
    ForthTwoDrop,  // 2DROP
    ForthTwoSwap,  // 2SWAP
    ForthTwoOver,  // 2OVER

    // Return stack operations
    ForthToR,      // >R ( n -- ) ( R: -- n )
    ForthFromR,    // R> ( -- n ) ( R: n -- )
    ForthRFetch,   // R@ ( -- n ) ( R: n -- n )

    // Arithmetic operations (reuse existing Add, Sub, Mul, Div when possible)
    ForthMod,      // MOD
    ForthDivMod,   // /MOD
    ForthMulDiv,   // */
    ForthMulDivMod,// */MOD
    ForthOnePlus,  // 1+
    ForthOneMinus, // 1-
    ForthTwoMul,   // 2*
    ForthTwoDiv,   // 2/
    ForthAbs,      // ABS
    ForthNegate,   // NEGATE (can reuse Neg)
    ForthMin,      // MIN
    ForthMax,      // MAX

    // Bitwise operations (can reuse And, Or when possible)
    ForthXor,      // XOR
    ForthInvert,   // INVERT
    ForthLShift,   // LSHIFT
    ForthRShift,   // RSHIFT

    // Comparison operations (can reuse Eq, LE when possible)
    ForthNotEq,    // <>
    ForthLessThan, // <
    ForthGreater,  // >
    ForthGreaterEq,// >=
    ForthZeroEq,   // 0=
    ForthZeroNotEq,// 0<>
    ForthZeroLess, // 0<
    ForthZeroGreater, // 0>

    // Memory operations
    ForthFetch,    // @ ( addr -- n )
    ForthStore,    // ! ( n addr -- )
    ForthPlusStore,// +! ( n addr -- )
    ForthCFetch,   // C@ ( addr -- c )
    ForthCStore,   // C! ( c addr -- )
    ForthCells,    // CELLS ( n -- n*cell_size )
    ForthAllot,    // ALLOT ( n -- )
    ForthComma,    // , ( n -- )
    ForthCComma,   // C, ( c -- )
    ForthHere,     // HERE ( -- addr )

    // I/O operations
    ForthDot,      // . ( n -- )
    ForthEmit,     // EMIT ( c -- )
    ForthCR,       // CR ( -- )
    ForthSpace,    // SPACE ( -- )
    ForthSpaces {  // SPACES ( n -- )
        count: Box<AlephTree>,
    },
    ForthType {    // TYPE ( addr u -- )
        addr: Box<AlephTree>,
        count: Box<AlephTree>,
    },
    ForthKey,      // KEY ( -- c )
    ForthAccept {  // ACCEPT ( addr +n1 -- +n2 )
        addr: Box<AlephTree>,
        max_len: Box<AlephTree>,
    },
    ForthDotQuote {// ." text"
        text: String
    },
    ForthSQuote {  // S" text"
        text: String
    },

    // Control structures
    ForthBeginUntil {  // BEGIN...UNTIL
        body: Vec<Box<AlephTree>>,
        condition: Box<AlephTree>,
    },

    ForthBeginWhileRepeat {  // BEGIN...WHILE...REPEAT
        condition: Box<AlephTree>,
        while_body: Vec<Box<AlephTree>>,
        repeat_body: Vec<Box<AlephTree>>,
    },

    ForthBeginAgain {  // BEGIN...AGAIN (infinite loop)
        body: Vec<Box<AlephTree>>,
    },

    ForthDoLoop {      // DO...LOOP
        body: Vec<Box<AlephTree>>,
    },

    ForthDoPlusLoop {  // DO...+LOOP
        body: Vec<Box<AlephTree>>,
        increment: Box<AlephTree>,
    },

    ForthLeave,        // LEAVE

    ForthCase {        // CASE...ENDCASE
        #[serde(alias="whenClauses")]
        when_clauses: Vec<Box<AlephTree>>,
        default: Option<Vec<Box<AlephTree>>>,
    },

    ForthOf {          // OF...ENDOF
        value: Box<AlephTree>,
        body: Vec<Box<AlephTree>>,
    },

    // Loop index access
    ForthI,            // I (current loop index)
    ForthJ,            // J (outer loop index)

    // Execution tokens
    ForthTick {        // ' (tick)
        word: String,
    },

    ForthBracketTick { // [']
        word: String,
    },

    ForthExecute {     // EXECUTE ( xt -- )
        xt: Box<AlephTree>,
    },

    // Compilation words
    ForthLiteral {     // LITERAL
        value: Box<AlephTree>,
    },

    ForthPostpone {    // POSTPONE
        word: String,
    },

    ForthBracket,      // [ (enter interpretation mode)
    ForthBracketClose, // ] (enter compilation mode)

    ForthImmediate,    // IMMEDIATE

    ForthRecursive,    // RECURSIVE

    // Dictionary manipulation
    ForthForget {      // FORGET
        word: String,
    },

    ForthWords,        // WORDS (list all words)

    ForthSee {         // SEE (decompile word)
        word: String,
    },

    // String operations
    ForthEvaluate {    // EVALUATE ( addr u -- )
        addr: Box<AlephTree>,
        count: Box<AlephTree>,
    },

    // Conversion operations
    ForthSToD,         // S>D (single to double)
    ForthDToS,         // D>S (double to single)

    // Program control
    ForthQuit,         // QUIT (outer interpreter loop)
    ForthAbort,        // ABORT
    ForthAbortQuote {  // ABORT" message"
        message: String,
    },

    // Comment (Forth-style)
    ForthComment {     // ( comment )
        text: String,
    },

    ForthLineComment { // \ comment
        text: String,
    },

    // Hexadecimal number
    ForthHex {         // 0x prefix or HEX mode
        value: String,
    },

    // Program structure
    ForthProgram {
        definitions: Vec<Box<AlephTree>>,
    },

    ForthSequence {    // Sequence of Forth words
        words: Vec<Box<AlephTree>>,
    }
}

pub fn json_parse(source: String) -> AlephTree {
    serde_json::from_str(&source).unwrap()
}

pub fn to_json(ast: AlephTree) -> String {
    serde_json::to_string_pretty(&ast).unwrap()
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

impl fmt::Display for AlephTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            e => write!(f, "{:?}", e),
        }
    }
}

impl AlephTree {
    pub fn to_string_value(&self) -> String {
        match self {
            AlephTree::Bool { value } => value.to_string(),
            AlephTree::Int { value } => value.to_string(),
            AlephTree::Float { value } => value.to_string(),
            AlephTree::String { value } => value.to_string(),
            AlephTree::Ident { value } => value.to_string(),
            AlephTree::Bytes { elems } => match std::str::from_utf8(elems) {
                Ok(s) => s.to_string(),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            },
            AlephTree::Figurative { figurative_type } => figurative_type.to_string(),
            AlephTree::HexLiteral { value } => value.to_string(),
            _ => {
                println!("Can't evaluate to_string_value : {}", self);
                panic!()
            }
        }
    }
}
