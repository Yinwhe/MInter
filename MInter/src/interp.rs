/*
 * @Author: Yinwhe
 * @Date: 2021-10-10 19:45:12
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-10-10 20:27:29
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

use crate::parser::is_literal;
pub use crate::syntax::Expr::{self, *};
pub use crate::syntax::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn interp_exp(expr: Expr, env: Rc<RefCell<SymTable<String, ValType>>>) -> ValType {
    match expr {
        Value(v) => v,
        Var(x) => env.borrow().lookup(&x).clone(),
        Make(box Var(x), box e) => {
            if ! is_literal(&x) {
                panic!("Make error, variable not a literal");
            }
            let val = interp_exp(e, Rc::clone(&env));
            env.borrow_mut().bind(x, val);
            ValType::Int(0)
        },
        Print(box data) => {
            let val = interp_exp(data, Rc::clone(&env));
            match val {
                ValType::Int(n) => println!("{}", n),
                ValType::Str(str) => println!("{}", str)
            }
            ValType::Int(0)
        },
        _ => panic!("bad syntax!"),
    }
}