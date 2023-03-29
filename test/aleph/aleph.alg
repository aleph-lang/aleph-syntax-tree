simple_exp:
    | "(" exp ")" -> $2
    | "{" exp "}" -> $2
    | "(" ")"     -> Unit
    | BOOL        -> Bool($1)
    | int         -> Int($1)
    | int "." int -> Float($1, $3)
    | "!" IDENT   -> Var($2, true)
    | IDENT       -> Var($1,false)
    | STRING      -> String($1)

exp:
    | ";" -> Unit
    | simple_exp -> $1
    | "!" exp -> Not($2)
    | "-" exp -> Neg($2)
    | exp "&" exp -> And($1,$3)
    | exp "|" exp -> Or($1,$3)
    | exp "+" exp -> Add($1,$3)
    | exp "-" exp -> Sub($1,$3)
    | exp "*" exp -> Mul($1,$3)
    | exp "/" exp -> Div($1, $3)
    | exp "=" exp -> Eq($1, $3)
    | exp "<" ">" exp -> Not(Eq($1, $3))
    | exp "<" exp -> Not(LE($3, $1))
    | exp ">" exp -> Not(LE($1, $3))
    | exp "<" "=" exp -> LE($1, $3)
    | exp ">" "=" exp -> LE($3, $1)
    | exp "?" exp -> If($1, $3, Unit)
    | exp "?" exp ":" exp -> If($1, $3, $5)
    | exp "?" "*" exp -> While(Unit, $1, $4, Unit)
    | "(" exp ";" exp ";" exp ")" "?" "*" exp -> While($2, $4, $10, $6)
    | "import" STRING -> Import($2)
    | "fun" IDENT actual_args "=" "{" exp "}" -> LetRec($2, $3, $6)
    | simple_exp actual_args -> App("", $1, $2)
    | IDENT "." simple_exp actual_args -> App($1, $3, $4)
    | elems -> Tuple($1)
    | IDENT "=" exp -> Let($1, false, $3, Unit)
    | IDENT ":" "=" exp -> Let($1, true, $4, Unit)
    | IDENT "=" exp ";" exp -> Let($1, false, $3, $5)
    | IDENT ":" "=" exp ";" exp -> Let($1, true, $4, $6)
    | "[" elems "]" -> Array($2)
    | IDENT "[" exp "]" "=" exp -> Put($1,$3,$6,false)
    | IDENT "[" exp "+" "]" "=" exp -> Put($1,$3,$7,true)
    | IDENT "[" exp "]" -> Get($1,$3)
    | IDENT "/" exp -> Remove($1,$3, false, false)
    | IDENT "/" exp BOOL -> Remove($1,$3, false, $4)
    | IDENT "-" exp -> Remove($1,$3, true, false)
    | IDENT "-" exp BOOL -> Remove($1,$3, true, $4)
    | "|" IDENT "|" -> Length($2)
    | "class" IDENT "{" exp "}" -> Class($2, $4) 
    | "\0" -> Unit
    | "match" exp "with" expMatch -> Match($2, $4)
    | exp ";" ->  $1
    | exp ";" exp -> Stmts($1, $3)

expMatchOne:
    | ":" exp "->" exp -> MatchLine($2, $4)

expMatch:
    | expMatch expMatchOne -> $1 @ [$2]
    | expMatchOne -> [$1]

actual_args:
    | actual_args simple_exp -> $1 @ [$2]
    | simple_exp -> [$1]

elems:
    | elems "," exp -> $1 @ [$3]
    | exp "," exp -> [$1; $3]
