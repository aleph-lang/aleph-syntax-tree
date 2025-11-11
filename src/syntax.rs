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
    Int{value: String},
    Float{value: String},
    Bool{value: String},
    String{value: String},
    Ident{value: String},
    Bytes{elems: Vec<u8>},
    Complex{real: String, imag: String},
    HexLiteral{value: String},
    Figurative{
        #[serde(alias="figurativeType")]
        figurative_type: String
    },
    Tuple{elems: Vec<Box<AlephTree>>},
    Array{elems: Vec<Box<AlephTree>>},
    Record{fields: Vec<Box<AlephTree>>},
    Field{name: String, value: Box<AlephTree>},
    Neg{expr: Box<AlephTree>},
    Not{
        #[serde(alias="boolExpr")]
        bool_expr: Box<AlephTree>
    },
    BitNot{expr: Box<AlephTree>},
    Abs{expr: Box<AlephTree>},
    Add{
        #[serde(alias="numberExpr1")]
        number_expr1: Box<AlephTree>,
        #[serde(alias="numberExpr2")]
        number_expr2: Box<AlephTree>
    },
    Sub{
        #[serde(alias="numberExpr1")]
        number_expr1: Box<AlephTree>,
        #[serde(alias="numberExpr2")]
        number_expr2: Box<AlephTree>
    },
    Mul{
        #[serde(alias="numberExpr1")]
        number_expr1: Box<AlephTree>,
        #[serde(alias="numberExpr2")]
        number_expr2: Box<AlephTree>
    },
    Div{
        #[serde(alias="numberExpr1")]
        number_expr1: Box<AlephTree>,
        #[serde(alias="numberExpr2")]
        number_expr2: Box<AlephTree>
    },
    Mod{
        #[serde(alias="numberExpr1")]
        number_expr1: Box<AlephTree>,
        #[serde(alias="numberExpr2")]
        number_expr2: Box<AlephTree>
    },
    DivMod{
        #[serde(alias="numberExpr1", alias="dividend")]
        dividend: Box<AlephTree>,
        #[serde(alias="numberExpr2", alias="divisor")]
        divisor: Box<AlephTree>
    },
    MulDiv{
        n1: Box<AlephTree>,
        n2: Box<AlephTree>,
        n3: Box<AlephTree>
    },
    MulDivMod{
        n1: Box<AlephTree>,
        n2: Box<AlephTree>,
        n3: Box<AlephTree>
    },
    Pow{base: Box<AlephTree>, exponent: Box<AlephTree>},
    Min{expr1: Box<AlephTree>, expr2: Box<AlephTree>},
    Max{expr1: Box<AlephTree>, expr2: Box<AlephTree>},
    And{
        #[serde(alias="boolExpr1")]
        bool_expr1: Box<AlephTree>,
        #[serde(alias="boolExpr2")]
        bool_expr2: Box<AlephTree>
    },
    Or{
        #[serde(alias="boolExpr1")]
        bool_expr1: Box<AlephTree>,
        #[serde(alias="boolExpr2")]
        bool_expr2: Box<AlephTree>
    },
    Xor{expr1: Box<AlephTree>, expr2: Box<AlephTree>},
    BitAnd{expr1: Box<AlephTree>, expr2: Box<AlephTree>},
    BitOr{expr1: Box<AlephTree>, expr2: Box<AlephTree>},
    BitXor{expr1: Box<AlephTree>, expr2: Box<AlephTree>},
    LShift{expr: Box<AlephTree>, amount: Box<AlephTree>},
    RShift{expr: Box<AlephTree>, amount: Box<AlephTree>},
    Eq{expr1: Box<AlephTree>, expr2: Box<AlephTree>},
    NotEq{expr1: Box<AlephTree>, expr2: Box<AlephTree>},
    LT{expr1: Box<AlephTree>, expr2: Box<AlephTree>},
    LE{expr1: Box<AlephTree>, expr2: Box<AlephTree>},
    GT{expr1: Box<AlephTree>, expr2: Box<AlephTree>},
    GE{expr1: Box<AlephTree>, expr2: Box<AlephTree>},
    In{expr1: Box<AlephTree>, expr2: Box<AlephTree>},
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
        post_expr: Box<AlephTree>
    },
    For{
        var: String,
        start: Box<AlephTree>,
        end: Box<AlephTree>,
        step: Option<Box<AlephTree>>,
        body: Box<AlephTree>,
        reverse: bool
    },
    Loop{
        name: Option<String>,
        body: Vec<Box<AlephTree>>
    },
    DoLoop{
        #[serde(alias="loopVar")]
        loop_var: Option<String>,
        #[serde(alias="startIndex")]
        start: Option<Box<AlephTree>>,
        #[serde(alias="endIndex")]
        end: Option<Box<AlephTree>>,
        by: Option<Box<AlephTree>>,
        #[serde(alias="whileCond")]
        while_cond: Option<Box<AlephTree>>,
        #[serde(alias="untilCond")]
        until_cond: Option<Box<AlephTree>>,
        body: Vec<Box<AlephTree>>
    },
    Case{
        expr: Box<AlephTree>,
        #[serde(alias="caseList", alias="whenClauses", alias="alternatives", alias="ofClauses")]
        cases: Vec<Box<AlephTree>>,
        #[serde(alias="whenOther", alias="otherwise", alias="default")]
        default_case: Option<Box<AlephTree>>
    },
    CaseBranch{
        #[serde(alias="condition", alias="selectionObject", alias="choices", alias="value")]
        pattern: Box<AlephTree>,
        #[serde(alias="caseExpr", alias="body", alias="statements")]
        body: Vec<Box<AlephTree>>
    },
    Match{
        expr: Box<AlephTree>,
        #[serde(alias="caseList")]
        case_list: Vec<Box<AlephTree>>
    },
    MatchLine{
        condition: Box<AlephTree>,
        #[serde(alias="caseExpr")]
        case_expr: Box<AlephTree>
    },
    ProcedureDef{
        name: String,
        #[serde(alias="procedureType")]
        proc_type: Option<String>, // "procedure", "function", "word", "entry", "main"
        parameters: Vec<Box<AlephTree>>,
        #[serde(alias="returnType")]
        return_type: Option<Box<AlephTree>>,
        attributes: Vec<String>,
        declarations: Vec<Box<AlephTree>>,
        body: Vec<Box<AlephTree>>
    },
    Parameter{
        name: String,
        #[serde(alias="paramType", alias="param_type")]
        param_type: Option<Box<AlephTree>>,
        mode: Option<String>, // "in", "out", "in out", "ref", "value", "by reference"
        default: Option<Box<AlephTree>>
    },
    VarDecl{
        name: String,
        #[serde(alias="levelNumber")]
        level: Option<String>, // For COBOL level numbers
        #[serde(alias="varType", alias="var_type", alias="objectType", alias="picture")]
        var_type: Option<Box<AlephTree>>,
        #[serde(alias="initialValue", alias="initial_value")]
        initial_value: Option<Box<AlephTree>>,
        #[serde(alias="isConstant", alias="is_constant")]
        is_constant: bool,
        #[serde(alias="isAliased", alias="is_aliased")]
        is_aliased: bool,
        storage: Option<String>,
        #[serde(alias="occursClause")]
        occurs: Option<String>,
        usage: Option<String>,
        attributes: Vec<String>
    },
    StructDecl{
        name: String,
        #[serde(alias="levelNumber")]
        level: Option<String>,
        members: Vec<Box<AlephTree>>,
        attributes: Vec<String>
    },
    TypeDecl{
        name: String,
        definition: Box<AlephTree>,
        attributes: Vec<String>
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
    Var{
        var: String,
        #[serde(alias="isPointer")]
        is_pointer: String
    },
    TypeRef{
        name: String,
        #[serde(alias="qualifierList")]
        qualifiers: Vec<String>
    },
    ArrayType{
        #[serde(alias="elementType")]
        element_type: Box<AlephTree>,
        dimensions: Vec<Box<AlephTree>>
    },
    PointerType{
        #[serde(alias="targetType")]
        target_type: Box<AlephTree>,
        #[serde(alias="isAllAccess")]
        is_all: bool
    },
    RangeType{
        start: Box<AlephTree>,
        end: Box<AlephTree>
    },
    EnumType{
        values: Vec<String>
    },
    RecordType{
        fields: Vec<Box<AlephTree>>
    },
    FieldDecl{
        name: String,
        #[serde(alias="fieldType")]
        field_type: Box<AlephTree>,
        attributes: Vec<String>
    },
    SubtypeDecl{
        name: String,
        #[serde(alias="baseType")]
        base_type: Box<AlephTree>,
        constraint: Option<Box<AlephTree>>
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
        is_value: String
    },
    Length{var: String},
    Slice{
        array: Box<AlephTree>,
        start: Option<Box<AlephTree>>,
        end: Option<Box<AlephTree>>
    },
    Alloc{
        var: Box<AlephTree>,
        #[serde(alias="setPointer", alias="typeOrExpr")]
        target: Option<Box<AlephTree>>
    },
    Free{
        var: Box<AlephTree>
    },
    Fetch{
        addr: Box<AlephTree>,
        #[serde(alias="fetchType")]
        fetch_type: Option<String> // "byte", "word", "cell"
    },
    Store{
        value: Box<AlephTree>,
        addr: Box<AlephTree>,
        #[serde(alias="storeType")]
        store_type: Option<String> // "byte", "word", "cell"
    },
    StoreOp{
        value: Box<AlephTree>,
        addr: Box<AlephTree>,
        op: String // "+", "-", etc. for +=, -=
    },
    App{
        #[serde(alias="objectName")]
        object_name: String,
        fun: Box<AlephTree>,
        #[serde(alias="paramList")]
        param_list: Vec<Box<AlephTree>>
    },
    Call{
        #[serde(alias="programName", alias="entryName")]
        target: Box<AlephTree>,
        #[serde(alias="usingParameters")]
        parameters: Option<Vec<Box<AlephTree>>>,
        #[serde(alias="givingParameter")]
        returning: Option<Box<AlephTree>>,
        #[serde(alias="onException")]
        on_error: Option<Box<AlephTree>>
    },
    Stmts{expr1: Box<AlephTree>, expr2: Box<AlephTree>},
    Block{statements: Vec<Box<AlephTree>>},
    Assignment{target: Box<AlephTree>, value: Box<AlephTree>},
    #[serde(alias="Return")]
    Return{value: Box<AlephTree>},
    GoTo{
        #[serde(alias="targetParagraph", alias="target")]
        target: String,
        #[serde(alias="dependingOn")]
        depending_on: Option<Box<AlephTree>>
    },
    Label{
        name: String,
        #[serde(alias="labelType")]
        label_type: Option<String> // "paragraph", "section", "entry"
    },
    Exit,
    Move{
        source: Box<AlephTree>,
        #[serde(alias="targetList")]
        targets: Vec<Box<AlephTree>>
    },
    Compute{
        target: Box<AlephTree>,
        expression: Box<AlephTree>,
        #[serde(alias="onSizeError")]
        on_error: Option<Box<AlephTree>>
    },
    Print{
        #[serde(alias="itemList", alias="items")]
        items: Vec<Box<AlephTree>>,
        #[serde(alias="uponDevice", alias="toFile")]
        destination: Option<String>,
        #[serde(alias="formatItems")]
        format: Option<Vec<Box<AlephTree>>>,
        options: Vec<String>
    },
    Input{
        #[serde(alias="target", alias="variables")]
        targets: Vec<Box<AlephTree>>,
        #[serde(alias="fromDevice", alias="fromFile")]
        source: Option<String>,
        options: Vec<String>
    },
    FileOpen{
        #[serde(alias="fileName", alias="file")]
        file: String,
        mode: String,
        attributes: Vec<String>
    },
    FileClose{
        #[serde(alias="fileList", alias="files")]
        files: Vec<String>
    },
    FileRead{
        #[serde(alias="fileName")]
        file_name: String,
        #[serde(alias="intoClause")]
        into: Option<Box<AlephTree>>,
        #[serde(alias="keyClause")]
        key: Option<Box<AlephTree>>,
        #[serde(alias="atEndClause", alias="onEnd")]
        on_end: Option<Box<AlephTree>>,
        #[serde(alias="notAtEndClause")]
        not_on_end: Option<Box<AlephTree>>
    },
    FileWrite{
        #[serde(alias="recordName")]
        record_name: String,
        #[serde(alias="fromClause")]
        from: Option<Box<AlephTree>>,
        #[serde(alias="advancingClause")]
        options: Vec<String>
    },
    StringOp{
        operation: String, // "concat", "inspect", "unstring", "replace"
        #[serde(alias="sourceItems", alias="sourceItem")]
        sources: Vec<Box<AlephTree>>,
        #[serde(alias="delimitedBy")]
        delimiter: Option<Box<AlephTree>>,
        #[serde(alias="intoItem", alias="intoItems")]
        targets: Vec<Box<AlephTree>>,
        #[serde(alias="withPointer")]
        pointer: Option<Box<AlephTree>>,
        #[serde(alias="onOverflow", alias="tallyingClause", alias="replacingClause")]
        clauses: Vec<Box<AlephTree>>
    },
    #[serde(alias="Import")]
    Iprt{
        name: String,
        items: Vec<String>
    },
    Module{
        name: String,
        #[serde(alias="moduleType")]
        module_type: String, // "package", "program", "unit", "interface", "spec", "body"
        #[serde(alias="programId")]
        id: Option<String>,
        declarations: Vec<Box<AlephTree>>,
        body: Option<Vec<Box<AlephTree>>>,
        initialization: Option<Vec<Box<AlephTree>>>
    },
    Division{
        #[serde(alias="divisionType")]
        division_type: String, // "environment", "data", "procedure"
        sections: Vec<Box<AlephTree>>
    },
    Section{
        name: String,
        #[serde(alias="sectionType")]
        section_type: Option<String>,
        content: Vec<Box<AlephTree>>
    },
    #[serde(alias="Class")]
    Clss{
        name: String,
        #[serde(alias="attributList")]
        attribute_list: Vec<String>,
        body: Box<AlephTree>
    },
    TryCatch{
        #[serde(alias="tryBlock")]
        try_block: Box<AlephTree>,
        #[serde(alias="catchClauses", alias="handlers")]
        catch_clauses: Vec<Box<AlephTree>>,
        #[serde(alias="finallyBlock")]
        finally_block: Option<Box<AlephTree>>
    },
    CatchClause{
        #[serde(alias="exceptionType", alias="exceptionChoices")]
        exception_types: Vec<String>,
        var: Option<String>,
        body: Box<AlephTree>
    },
    OnCondition{
        condition: String, // "SIZE", "OVERFLOW", "ENDFILE", etc.
        #[serde(alias="snapOption")]
        options: Vec<String>,
        handler: Box<AlephTree>
    },
    Raise{
        #[serde(alias="exception")]
        condition: Box<AlephTree>
    },
    Signal{
        condition: String
    },
    Revert{
        condition: String
    },
    ExceptionDecl{
        name: String
    },
    #[serde(alias="Comment")]
    Comment{value: String},
    #[serde(alias="CommentMulti")]
    CommentMulti{value: String},
    Assert{
        condition: Box<AlephTree>,
        message: Box<AlephTree>
    },
    Generic{
        name: String,
        #[serde(alias="genericType")]
        generic_type: String,
        #[serde(alias="genericParams")]
        generic_params: Vec<Box<AlephTree>>,
        body: Box<AlephTree>
    },
    GenericParam{
        name: String,
        #[serde(alias="paramKind")]
        param_kind: String,
        constraint: Option<Box<AlephTree>>,
        default: Option<Box<AlephTree>>
    },
    Instantiation{
        name: String,
        #[serde(alias="genericName")]
        generic_name: String,
        #[serde(alias="actualParams")]
        actual_params: Vec<Box<AlephTree>>
    },
    Attribute{
        prefix: Box<AlephTree>,
        attribute: String,
        args: Option<Vec<Box<AlephTree>>>
    },
    Pragma{
        name: String,
        args: Vec<Box<AlephTree>>
    },
    RepresentationClause{
        name: String,
        #[serde(alias="clauseType")]
        clause_type: String,
        specification: Box<AlephTree>
    },
    Renaming{
        #[serde(alias="newName")]
        new_name: String,
        #[serde(alias="renamedEntity", alias="redefinedItem")]
        old_name: Box<AlephTree>,
        #[serde(alias="entityType")]
        rename_type: Option<Box<AlephTree>>
    },
    Execute{
        target: Box<AlephTree>,
        #[serde(alias="executionType")]
        exec_type: Option<String> // "word", "token", "xt"
    },
    Perform{
        #[serde(alias="targetParagraph")]
        target: Option<String>,
        #[serde(alias="fromParagraph")]
        from: Option<String>,
        #[serde(alias="throughParagraph")]
        through: Option<String>,
        #[serde(alias="timesClause")]
        times: Option<Box<AlephTree>>,
        #[serde(alias="untilClause")]
        until: Option<Box<AlephTree>>,
        #[serde(alias="varyingClause")]
        varying: Option<Box<AlephTree>>,
        #[serde(alias="inlineStatements")]
        inline: Option<Vec<Box<AlephTree>>>
    },
    // ===== COBOL-specific (irreducible) =====
    PicClause{
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
    FileDescription{
        #[serde(alias="fileName")]
        file_name: String,
        #[serde(alias="recordDescription")]
        record_description: Vec<Box<AlephTree>>,
        #[serde(alias="blockContains")]
        block_contains: Option<String>,
        #[serde(alias="recordContains")]
        record_contains: Option<String>
    },
    SelectStatement{
        #[serde(alias="fileName")]
        file_name: String,
        #[serde(alias="assignTo")]
        assign_to: String,
        #[serde(alias="accessMode")]
        access_mode: Option<String>,
        #[serde(alias="organizationMode")]
        organization_mode: Option<String>
    },
    Inspect{
        #[serde(alias="inspectingItem")]
        inspecting_item: Box<AlephTree>,
        #[serde(alias="tallyingClause")]
        tallying_clause: Option<Box<AlephTree>>,
        #[serde(alias="replacingClause")]
        replacing_clause: Option<Box<AlephTree>>
    },
    Accept{target: Box<AlephTree>, #[serde(alias="fromDevice")] from_device: Option<String>},
    Display{#[serde(alias="itemList")] item_list: Vec<Box<AlephTree>>, #[serde(alias="uponDevice")] upon_device: Option<String>},
    Open{mode: String, #[serde(alias="fileList")] file_list: Vec<String>},
    Close{#[serde(alias="fileList")] file_list: Vec<String>},
    Read{
        #[serde(alias="fileName")] file_name: String,
        #[serde(alias="intoClause")] into_clause: Option<Box<AlephTree>>,
        #[serde(alias="keyClause")] key_clause: Option<Box<AlephTree>>,
        #[serde(alias="atEndClause")] at_end_clause: Option<Box<AlephTree>>,
        #[serde(alias="notAtEndClause")] not_at_end_clause: Option<Box<AlephTree>>
    },
    Write{
        #[serde(alias="recordName")] record_name: String,
        #[serde(alias="fromClause")] from_clause: Option<Box<AlephTree>>,
        #[serde(alias="advancingClause")] advancing_clause: Option<String>
    },
    Stop{#[serde(alias="stopType")] stop_type: String},
    Paragraph{name: String, statements: Vec<Box<AlephTree>>},
    Evaluate{
        #[serde(alias="selectionSubject")] selection_subject: Box<AlephTree>,
        #[serde(alias="whenClauses")] when_clauses: Vec<Box<AlephTree>>,
        #[serde(alias="whenOther")] when_other: Option<Box<AlephTree>>
    },
    WhenClause{
        #[serde(alias="selectionObject")] selection_object: Box<AlephTree>,
        statements: Vec<Box<AlephTree>>
    },
    StringStmt{
        #[serde(alias="sourceItems")] source_items: Vec<Box<AlephTree>>,
        #[serde(alias="delimitedBy")] delimited_by: Box<AlephTree>,
        #[serde(alias="intoItem")] into_item: Box<AlephTree>,
        #[serde(alias="withPointer")] with_pointer: Option<Box<AlephTree>>,
        #[serde(alias="onOverflow")] on_overflow: Option<Box<AlephTree>>
    },
    Unstring{
        #[serde(alias="sourceItem")] source_item: Box<AlephTree>,
        #[serde(alias="delimitedBy")] delimited_by: Box<AlephTree>,
        #[serde(alias="intoItems")] into_items: Vec<Box<AlephTree>>,
        #[serde(alias="withPointer")] with_pointer: Option<Box<AlephTree>>,
        #[serde(alias="onOverflow")] on_overflow: Option<Box<AlephTree>>
    },
    OnSizeError{statements: Vec<Box<AlephTree>>},
    NotOnSizeError{statements: Vec<Box<AlephTree>>},
    QualifiedName{
        #[serde(alias="dataName")] data_name: String,
        #[serde(alias="qualifierList")] qualifier_list: Vec<String>
    },
    Subscript{
        #[serde(alias="dataName")] data_name: String,
        #[serde(alias="subscriptList")] subscript_list: Vec<Box<AlephTree>>
    },
    ClassCondition{
        #[serde(alias="dataItem")] data_item: Box<AlephTree>,
        #[serde(alias="className")] class_name: String
    },
    SignCondition{
        #[serde(alias="dataItem")] data_item: Box<AlephTree>,
        sign: String
    },
    OccursClause{
        #[serde(alias="minOccurs")] min_occurs: Option<String>,

        #[serde(alias="maxOccurs")] max_occurs: String,
        #[serde(alias="dependingOn")] depending_on: Option<String>,
        #[serde(alias="indexedBy")] indexed_by: Option<Vec<String>>
    },
    UsageClause{
        #[serde(alias="usageType")] usage_type: String
    },
    ForthProgram{definitions: Vec<Box<AlephTree>>},
    ForthSequence{words: Vec<Box<AlephTree>>},
    ForthDup, ForthDrop, ForthSwap, ForthOver, ForthRot,
    ForthMinusRot, ForthNip, ForthTuck,
    ForthPick{depth: Box<AlephTree>},
    ForthRoll{depth: Box<AlephTree>},
    ForthTwoDup, ForthTwoDrop, ForthTwoSwap, ForthTwoOver,
    ForthToR, ForthFromR, ForthRFetch,
    ForthCells{n: Box<AlephTree>},
    ForthAllot{n: Box<AlephTree>},
    ForthComma{value: Box<AlephTree>},
    ForthCComma{value: Box<AlephTree>},
    ForthHere,
    ForthDot{value: Box<AlephTree>},
    ForthEmit{char: Box<AlephTree>},
    ForthCR, ForthSpace,
    ForthSpaces{count: Box<AlephTree>},
    ForthType{addr: Box<AlephTree>, count: Box<AlephTree>},
    ForthKey,
    ForthAccept{addr: Box<AlephTree>, #[serde(alias="maxLen")] max_len: Box<AlephTree>},
    ForthDotQuote{text: String},
    ForthSQuote{text: String},
    ForthBeginUntil{body: Vec<Box<AlephTree>>, condition: Box<AlephTree>},
    ForthBeginWhileRepeat{
        condition: Box<AlephTree>,
        #[serde(alias="whileBody")] while_body: Vec<Box<AlephTree>>,
        #[serde(alias="repeatBody")] repeat_body: Vec<Box<AlephTree>>
    },
    ForthBeginAgain{body: Vec<Box<AlephTree>>},
    ForthLeave,
    ForthI, ForthJ,
    ForthTick{word: String},
    ForthBracketTick{word: String},
    ForthLiteral{value: Box<AlephTree>},
    ForthPostpone{word: String},
    ForthBracket, ForthBracketClose,
    ForthImmediate, ForthRecursive,
    ForthForget{word: String},
    ForthWords,
    ForthSee{word: String},
    ForthEvaluate{addr: Box<AlephTree>, count: Box<AlephTree>},
    ForthSToD{value: Box<AlephTree>},
    ForthDToS{value: Box<AlephTree>},
    ForthQuit, ForthAbort,
    ForthAbortQuote{message: String},
    ForthCreate{
        name: String,
        #[serde(alias="allotSize")] allot_size: Option<Box<AlephTree>>,
        #[serde(alias="doesBody")] does_body: Option<Vec<Box<AlephTree>>>
    },
    AdaTaskType{
        name: String,
        discriminants: Option<Vec<Box<AlephTree>>>,
        entries: Vec<Box<AlephTree>>,
        body: Vec<Box<AlephTree>>
    },
    AdaProtectedType{
        name: String,
        discriminants: Option<Vec<Box<AlephTree>>>,
        declarations: Vec<Box<AlephTree>>
    },
    AdaProtectedBody{
        name: String,
        bodies: Vec<Box<AlephTree>>
    },
    AdaAccept{
        #[serde(alias="entryName")] entry_name: String,
        parameters: Option<Vec<Box<AlephTree>>>,
        body: Option<Vec<Box<AlephTree>>>
    },
    AdaSelect{
        alternatives: Vec<Box<AlephTree>>,
        #[serde(alias="elseClause")] else_clause: Option<Vec<Box<AlephTree>>>
    },
    AdaSelectiveAccept{
        #[serde(alias="guardCondition")] guard_condition: Option<Box<AlephTree>>,
        #[serde(alias="acceptStmt")] accept_stmt: Box<AlephTree>,
        statements: Vec<Box<AlephTree>>
    },
    AdaDelay{
        #[serde(alias="delayType")] delay_type: String,
        expression: Box<AlephTree>
    },
    AdaAbort{tasks: Vec<String>},
    AdaAggregate{
        components: Vec<Box<AlephTree>>
    },
    AdaComponentAssoc{
        choices: Option<Vec<Box<AlephTree>>>,
        expression: Box<AlephTree>
    },
    AdaQualified{
        #[serde(alias="typeName")] type_name: Box<AlephTree>,
        expression: Box<AlephTree>
    },
    AdaWith{packages: Vec<String>},
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
