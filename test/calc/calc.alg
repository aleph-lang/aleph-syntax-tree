expr:
    | integer -> Integer($1)
    | expr "*" expr -> Mul($1, s2)
    | expr "/" expr -> Div($1, s2)
    | expr "+" expr -> Add($1, $2)
    | expr "-" expr -> Sub($1, $2)

integer:
    | r"-{0,1}[0-9]+" -> $1
