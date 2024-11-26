use crate::models::Expr;
use std::cell::RefCell;
use std::collections::HashSet;

// 自由変数の一覧を取得する
fn get_free_vars(e: &Expr) -> HashSet<String> {
    let mut free_vars = HashSet::new();
    let mut bound_vars = HashSet::new();

    fn get_free_vars_rec(e: &Expr, fv: &mut HashSet<String>, bv: &mut HashSet<String>) {
        match e {
            Expr::Var(v) => {
                // 同名の自由変数がなければ、自由変数
                if !bv.contains(v) {
                    fv.insert(v.clone());
                }
            }
            Expr::Abs(v, body) => {
                // すでに外のスコープで束縛されている変数は、束縛変数
                // ここで初めて束縛した場合は再帰を抜けた後は
                // 束縛変数として扱ってはだめなので取り除く
                let need_removing = bv.insert(v.clone());
                get_free_vars_rec(body, fv, bv);
                if need_removing {
                    bv.remove(v);
                }
            }
            Expr::App(l, r) => {
                get_free_vars_rec(l, fv, bv);
                get_free_vars_rec(r, fv, bv);
            }
        }
    }

    get_free_vars_rec(e, &mut free_vars, &mut bound_vars);
    free_vars
}

// static変数で、解消すべき変数名のカウンタを持っておく
thread_local!(static DISAMBIGUATE_CTR: RefCell<u64> = RefCell::new(0));
fn disambiguate(w: String) -> String {
    DISAMBIGUATE_CTR.with(|ctr| {
        let mut ctr = ctr.borrow_mut();
        *ctr += 1;
        format!("{}_{}", w, ctr)
    })
}

fn alpha_convert(v: String, body: Expr) -> (String, Expr) {
    let nv = disambiguate(v.clone());
    let nb = substitute(body, v, &Expr::Var(nv.clone()));
    (nv, nb)
}

fn substitute(root: Expr, var: String, val: &Expr) -> Expr {
    let free_vars = get_free_vars(val);
    match root {
        Expr::Var(v) => {
            if v == var {
                val.clone()
            } else {
                Expr::Var(v)
            }
        }
        Expr::Abs(v, body) => {
            if v == var {
                Expr::Abs(v, body)
            } else if free_vars.contains(&v) {
                let (nv, nb) = alpha_convert(v, *body);
                Expr::Abs(nv, Box::new(substitute(nb, var, val)))
            } else {
                Expr::Abs(v, Box::new(substitute(*body, var, val)))
            }
        }
        Expr::App(l, r) => Expr::App(
            Box::new(substitute(*l, var.clone(), val)),
            Box::new(substitute(*r, var, val)),
        ),
    }
}

fn beta_reduce(abs: Expr, val: Expr) -> Expr {
    match abs {
        Expr::Abs(var, body) => substitute(*body, var, &val),
        _ => panic!("can't apply to non-abstraction"),
    }
}

pub fn reduce_expression(e: Expr) -> Expr {
    match e {
        Expr::App(l, r) => {
            let (l, r) = (reduce_expression(*l), reduce_expression(*r));
            match l {
                Expr::Abs(_, _) => reduce_expression(beta_reduce(l, r)),
                _ => Expr::App(Box::new(l), Box::new(r)),
            }
        }
        _ => e,
    }
}
