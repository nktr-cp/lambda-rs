## Production rule
```
expr ::= abstraction | application | term | "(" expr ")"
abstraction ::= lambda term "." expr
application ::= expr expr expr*
term ::= [A-Za-z]+
lambda ::= "\" | "λ"
```