use crate::models::Expr;
use std::cell::RefCell;
use std::collections::HashSet;

// expr中の自由変数の一覧を取得する
fn get_free_vars(expr: &Expr) -> HashSet<String> {
    fn collect_free_vars(
        expr: &Expr,
        free_vars: &mut HashSet<String>,
        bound_vars: &mut HashSet<String>,
    ) {
        match expr {
            Expr::Var(var_name) => {
                if !bound_vars.contains(var_name) {
                    free_vars.insert(var_name.clone());
                }
            }
            Expr::Abs(param, body) => {
                let is_new_binding = bound_vars.insert(param.clone());
                collect_free_vars(body, free_vars, bound_vars);
                if is_new_binding {
                    bound_vars.remove(param);
                }
            }
            Expr::App(left, right) => {
                collect_free_vars(left, free_vars, bound_vars);
                collect_free_vars(right, free_vars, bound_vars);
            }
        }
    }

    let mut free_vars = HashSet::new();
    let mut bound_vars = HashSet::new();
    collect_free_vars(expr, &mut free_vars, &mut bound_vars);
    free_vars
}

// static変数で、解消すべき変数名のカウンタを持っておく
// もし自由変数と同名の束縛変数があった場合、衝突を避けるためにsuffixをつける
thread_local!(static DISAMBIGUATE_CTR: RefCell<u64> = RefCell::new(0));
fn disambiguate(w: &str) -> String {
    DISAMBIGUATE_CTR.with(|ctr| {
        let mut ctr = ctr.borrow_mut();
        *ctr += 1;
        format!("{}_{}", w, ctr)
    })
}

fn alpha_convert(var: String, body: Expr) -> (String, Expr) {
    let new_vars = disambiguate(&var);
    let new_body = substitute(body, &var, &Expr::Var(new_vars.clone()));
    (new_vars, new_body)
}

// expr中の変数varをreplacementで置き換える
fn substitute(expr: Expr, var: &str, replacement: &Expr) -> Expr {
    let free_vars = get_free_vars(replacement);
    match expr {
        Expr::Var(var_name) => {
            if var_name == var {
                replacement.clone()
            } else {
                Expr::Var(var_name)
            }
        }
        Expr::Abs(param, body) => {
            if param == var {
                Expr::Abs(param, body)
            } else if free_vars.contains(&param) {
                let (new_var, new_body) = alpha_convert(param, *body);
                Expr::Abs(new_var, Box::new(substitute(new_body, var, replacement)))
            } else {
                Expr::Abs(param, Box::new(substitute(*body, var, replacement)))
            }
        }
        Expr::App(left, right) => Expr::App(
            Box::new(substitute(*left, var, replacement)),
            Box::new(substitute(*right, var, replacement)),
        ),
    }
}

fn beta_reduce(abs: Expr, replacement: Expr) -> Expr {
    if let Expr::Abs(param, body) = abs {
        substitute(*body, &param, &replacement)
    } else {
        panic!("can't apply to non-abstraction");
    }
}

pub fn reduce_expression(expr: Expr) -> Expr {
    match expr {
        Expr::App(left, right) => {
            let reduced_left = reduce_expression(*left);
            let reduced_right = reduce_expression(*right);

            if let Expr::Abs(var, body) = reduced_left {
                return reduce_expression(beta_reduce(Expr::Abs(var, body), reduced_right));
            } else {
                return Expr::App(Box::new(reduced_left), Box::new(reduced_right));
            }
        }
        _ => expr,
    }
}
