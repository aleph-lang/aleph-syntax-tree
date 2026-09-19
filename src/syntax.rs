use std::fmt;
use strum_macros::EnumString;
use serde::{Deserialize, Serialize};
use crate::types::Type;
use crate::effects::EffectSet;

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
        proc_type: Option<String>,
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
        mode: Option<String>,
        default: Option<Box<AlephTree>>
    },
    VarDecl{
        name: String,
        #[serde(alias="levelNumber")]
        level: Option<String>,
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
        fetch_type: Option<String>
    },
    Store{
        value: Box<AlephTree>,
        addr: Box<AlephTree>,
        #[serde(alias="storeType")]
        store_type: Option<String>
    },
    StoreOp{
        value: Box<AlephTree>,
        addr: Box<AlephTree>,
        op: String
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
        label_type: Option<String>
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
    FileConfig{
        #[serde(alias="fileName")]
        file_name: String,
        #[serde(alias="assignTo")]
        assign_to: Option<String>,
        #[serde(alias="accessMode")]
        access_mode: Option<String>,
        #[serde(alias="organizationMode")]
        organization_mode: Option<String>,
        #[serde(alias="recordDescription")]
        record_description: Option<Vec<Box<AlephTree>>>,
        #[serde(alias="blockContains")]
        block_contains: Option<String>,
        #[serde(alias="recordContains")]
        record_contains: Option<String>
    },
    StringOp{
        operation: String,
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
    Export{
        items: Vec<Box<AlephTree>>,
        #[serde(alias="exportType")]
        export_type: Option<String>
    },
    Module{
        name: String,
        #[serde(alias="moduleType")]
        module_type: String,
        #[serde(alias="programId")]
        id: Option<String>,
        declarations: Vec<Box<AlephTree>>,
        body: Option<Vec<Box<AlephTree>>>,
        initialization: Option<Vec<Box<AlephTree>>>
    },
    Division{
        #[serde(alias="divisionType")]
        division_type: String,
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
        #[serde(alias="extends")]
        extends: Option<Box<AlephTree>>,
        #[serde(alias="implements")]
        implements: Vec<Box<AlephTree>>,
        body: Box<AlephTree>
    },
    New{
        constructor: Box<AlephTree>,
        args: Vec<Box<AlephTree>>
    },
    This,
    Super{
        #[serde(alias="memberAccess")]
        member: Option<String>
    },
    Member{
        object: Box<AlephTree>,
        member: String,
        #[serde(alias="isOptional")]
        is_optional: bool
    },
    Spread{
        expr: Box<AlephTree>,
        #[serde(alias="spreadType")]
        spread_type: String
    },
    Destructure{
        pattern: Box<AlephTree>,
        value: Box<AlephTree>,
        #[serde(alias="destructureType")]
        destructure_type: String
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
        condition: String,
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
        exec_type: Option<String>
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
    StackOp{
        operation: String,
        #[serde(alias="operands")]
        args: Vec<Box<AlephTree>>
    },
    TaskType{
        name: String,
        #[serde(alias="discriminants")]
        parameters: Option<Vec<Box<AlephTree>>>,
        entries: Vec<Box<AlephTree>>,
        body: Vec<Box<AlephTree>>
    },
    ProtectedType{
        name: String,
        #[serde(alias="discriminants")]
        parameters: Option<Vec<Box<AlephTree>>>,
        declarations: Vec<Box<AlephTree>>
    },
    ProtectedBody{
        name: String,
        bodies: Vec<Box<AlephTree>>
    },
    Accept{
        #[serde(alias="entryName")]
        entry_name: String,
        parameters: Option<Vec<Box<AlephTree>>>,
        body: Option<Vec<Box<AlephTree>>>
    },
    Select{
        alternatives: Vec<Box<AlephTree>>,
        #[serde(alias="elseClause")]
        else_clause: Option<Vec<Box<AlephTree>>>
    },
    SelectiveAccept{
        #[serde(alias="guardCondition")]
        guard_condition: Option<Box<AlephTree>>,
        #[serde(alias="acceptStmt")]
        accept_stmt: Box<AlephTree>,
        statements: Vec<Box<AlephTree>>
    },
    Delay{
        #[serde(alias="delayType")]
        delay_type: String,
        expression: Box<AlephTree>
    },
    Abort{
        targets: Vec<String>
    },
    Aggregate{
        components: Vec<Box<AlephTree>>
    },
    ComponentAssoc{
        choices: Option<Vec<Box<AlephTree>>>,
        expression: Box<AlephTree>
    },
    Qualified{
        #[serde(alias="typeName")]
        type_name: Box<AlephTree>,
        expression: Box<AlephTree>
    },
    MemoryOp{
        operation: String,
        #[serde(alias="operands")]
        args: Vec<Box<AlephTree>>
    },
    DataOp{
        operation: String,
        value: Box<AlephTree>
    },
    Emit{
        #[serde(alias="content")]
        value: Box<AlephTree>,
        #[serde(alias="emitType")]
        emit_type: Option<String>
    },
    Immediate,
    Recursive,
    Forget{
        target: String
    },
    Postpone{
        word: String
    },
    Literal{
        value: Box<AlephTree>
    },
    CompileTime{
        expression: Box<AlephTree>
    },
    CreateDoes{
        name: String,
        #[serde(alias="allotSize")]
        allot_size: Option<Box<AlephTree>>,
        #[serde(alias="doesBody")]
        does_body: Option<Vec<Box<AlephTree>>>
    },
    Introspect{
        operation: String,
        target: Option<String>
    },
    Convert{
        value: Box<AlephTree>,
        #[serde(alias="fromType")]
        from_type: String,
        #[serde(alias="toType")]
        to_type: String
    },
    Await{
        expr: Box<AlephTree>
    },
    Async{
        body: Box<AlephTree>
    },
    Yield{
        value: Option<Box<AlephTree>>
    },
    Typeof{
        expr: Box<AlephTree>
    },
    Instanceof{
        expr: Box<AlephTree>,
        type_expr: Box<AlephTree>
    },
    Nullish{
        expr: Box<AlephTree>,
        default: Box<AlephTree>
    },
    Optional{
        base: Box<AlephTree>,
        #[serde(alias="optionalType")]
        optional_type: String
    },
    Decorator{
        name: String,
        args: Vec<Box<AlephTree>>,
        target: Box<AlephTree>
    },
    TableOp{
        operation: String,
        table: Box<AlephTree>,
        index: Option<Box<AlephTree>>,
        value: Option<Box<AlephTree>>
    },
    Global{
        name: String,
        #[serde(alias="globalType")]
        global_type: Box<AlephTree>,
        mutable: bool,
        #[serde(alias="initialValue")]
        initial_value: Option<Box<AlephTree>>
    },
    Local{
        name: String,
        #[serde(alias="localType")]
        local_type: Box<AlephTree>
    },
    Br{
        depth: Box<AlephTree>,
        condition: Option<Box<AlephTree>>
    },
    Unreachable,
    Nop,
    UnionType{
        variants: Vec<Box<AlephTree>>
    },
    Trait{
        name: String,
        #[serde(alias="typeParams")]
        type_params: Vec<Box<AlephTree>>,
        #[serde(alias="superTraits")]
        super_traits: Vec<Box<AlephTree>>,
        items: Vec<Box<AlephTree>>
    },
    Impl{
        trait_ref: Option<Box<AlephTree>>,
        #[serde(alias="forType")]
        for_type: Box<AlephTree>,
        items: Vec<Box<AlephTree>>
    },
    Channel{
        name: String,
        #[serde(alias="channelType")]
        channel_type: Box<AlephTree>,
        capacity: Option<Box<AlephTree>>
    },
    Send{
        channel: Box<AlephTree>,
        value: Box<AlephTree>
    },
    Receive{
        channel: Box<AlephTree>,
        timeout: Option<Box<AlephTree>>
    },
    Spawn{
        target: Box<AlephTree>,
        args: Vec<Box<AlephTree>>
    },
    Macro{
        name: String,
        parameters: Vec<Box<AlephTree>>,
        body: Box<AlephTree>,
        #[serde(alias="macroType")]
        macro_type: Option<String>
    },
    MacroInvoke{
        name: String,
        args: Vec<Box<AlephTree>>
    },
    Pipe{
        expr: Box<AlephTree>,
        operations: Vec<Box<AlephTree>>
    },
    Comprehension{
        #[serde(alias="compType")]
        comp_type: String,
        expr: Box<AlephTree>,
        clauses: Vec<Box<AlephTree>>
    },
    CompClause{
        #[serde(alias="clauseType")]
        clause_type: String,
        pattern: Option<Box<AlephTree>>,
        iter: Option<Box<AlephTree>>,
        condition: Option<Box<AlephTree>>
    },
    With{
        resources: Vec<Box<AlephTree>>,
        body: Box<AlephTree>
    },
    Query{
        #[serde(alias="queryType")]
        query_type: String,
        #[serde(alias="selectClause")]
        select: Option<Box<AlephTree>>,
        #[serde(alias="fromClause")]
        from: Option<Vec<Box<AlephTree>>>,
        #[serde(alias="whereClause")]
        where_clause: Option<Box<AlephTree>>,
        #[serde(alias="joinClauses")]
        joins: Vec<Box<AlephTree>>,
        #[serde(alias="groupByClause")]
        group_by: Option<Vec<Box<AlephTree>>>,
        #[serde(alias="havingClause")]
        having: Option<Box<AlephTree>>,
        #[serde(alias="orderByClause")]
        order_by: Option<Vec<Box<AlephTree>>>,
        limit: Option<Box<AlephTree>>,
        offset: Option<Box<AlephTree>>
    },
    Join{
        #[serde(alias="joinType")]
        join_type: String,
        source: Box<AlephTree>,
        condition: Box<AlephTree>
    },
    Constraint{
        #[serde(alias="constraintType")]
        constraint_type: String,
        expressions: Vec<Box<AlephTree>>
    },
    Clause{
        head: Box<AlephTree>,
        body: Vec<Box<AlephTree>>
    },
    Unify{
        expr1: Box<AlephTree>,
        expr2: Box<AlephTree>
    },
    Lifetime{
        name: String,
        constraint: Option<Box<AlephTree>>
    },
    Borrow{
        expr: Box<AlephTree>,
        mutable: bool,
        lifetime: Option<String>
    },
    Unsafe{
        body: Box<AlephTree>
    },

    // ── Cognitive Layer ────────────────────────────────────────────────────
    // Intentional primitives: the program *reasons* instead of *executes*.
    // An intention block — like `fun` but driven by purpose, not procedure.
    Intend{
        name: String,
        params: Vec<Box<AlephTree>>,
        body: Box<AlephTree>
    },

    // Proposes a candidate from a context; optionally constrained by options.
    Suggest{
        var: String,
        context: Box<AlephTree>,
        options: Vec<Box<AlephTree>>,
        fallback: Option<Box<AlephTree>>
    },

    // Executes an intention and captures its observable effect.
    Act{
        intention: Box<AlephTree>,
        effect: Option<Box<AlephTree>>
    },

    // Persists a value in cognitive memory; ttl=None means permanent.
    Remember{
        key: Box<AlephTree>,
        value: Box<AlephTree>,
        ttl: Option<Box<AlephTree>>
    },

    // Reads a value from the environment matching an optional pattern.
    Perceive{
        source: Box<AlephTree>,
        pattern: Option<Box<AlephTree>>,
        var: Option<String>
    },

    // ── Type & Effect Layer (Aleph-Next) ────────────────────────────────────
    /// Attaches a static type to a subtree — e.g. a function parameter or a
    /// declared return position. Front-ends that don't type-check (every
    /// parser that predates Aleph-Next) simply never emit this node, and
    /// existing generators never need to handle it because they never
    /// receive a tree that contains one.
    Typed{
        inner: Box<AlephTree>,
        ty: Type
    },

    /// Declares a sum type: `type Shape = Circle(radius: Float) | Rect(w: Float, h: Float)`.
    /// Reuses `types::Variant` — the same shape `Type::Sum` uses — rather
    /// than a raw tuple, for the same reasons (named field access, cleaner
    /// wire format).
    TypeDef{
        name: String,
        variants: Vec<crate::types::Variant>
    },

    /// Declares the effect row of a function body (`pure`, `io`, `net`,
    /// `mut`, `act`). See `crate::effects::Effect`.
    WithEffects{
        inner: Box<AlephTree>,
        effects: EffectSet
    },
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effects::Effect;
    use crate::types::{Type, Variant};

    #[test]
    fn typed_node_round_trips_through_json() {
        let node = AlephTree::Typed {
            inner: Box::new(AlephTree::Ident { value: "n".to_string() }),
            ty: Type::Int,
        };
        let json = to_json(node.clone());
        let back = json_parse(json);
        assert_eq!(node, back);
    }

    #[test]
    fn typed_node_wire_format_is_locked() {
        let node = AlephTree::Typed {
            inner: Box::new(AlephTree::Ident { value: "n".to_string() }),
            ty: Type::Int,
        };
        assert_eq!(
            to_json(node),
            "{\n  \"type\": \"Typed\",\n  \"inner\": {\n    \"type\": \"Ident\",\n    \"value\": \"n\"\n  },\n  \"ty\": {\n    \"type\": \"Int\"\n  }\n}"
        );
    }

    #[test]
    fn type_def_round_trips_through_json() {
        let node = AlephTree::TypeDef {
            name: "Shape".to_string(),
            variants: vec![
                Variant { name: "Circle".to_string(), fields: vec![Type::Float] },
                Variant { name: "Rect".to_string(), fields: vec![Type::Float, Type::Float] },
            ],
        };
        let json = to_json(node.clone());
        let back = json_parse(json);
        assert_eq!(node, back);
    }

    #[test]
    fn with_effects_node_round_trips_through_json() {
        let node = AlephTree::WithEffects {
            inner: Box::new(AlephTree::App {
                object_name: "".to_string(),
                fun: Box::new(AlephTree::Ident { value: "print".to_string() }),
                param_list: Vec::new(),
            }),
            effects: crate::effects::EffectSet::from([Effect::Io, Effect::Net]),
        };
        let json = to_json(node.clone());
        let back = json_parse(json);
        assert_eq!(node, back);
    }
}
