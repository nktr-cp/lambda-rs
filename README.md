## Production rule
```
expr ::= abstraction | application | term | "(" expr ")"
abstraction ::= lambda term "." expr
application ::= expr expr expr*
term ::= [A-Za-z]+
lambda ::= "\" | "λ"
```

## Acknowledgments
- [Writing a lambda calculus interpreter in Rust](https://prose.nsood.in/rust-lambda)
