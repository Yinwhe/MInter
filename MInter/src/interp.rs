/*
 * @Author: Yinwhe
 * @Date: 2021-10-10 19:45:12
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-10-11 14:49:38
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

pub use crate::syntax::Expr::{self, *};
pub use crate::syntax::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn interp_exp(expr: Expr, env: Rc<RefCell<SymTable<String, ValType>>>) -> ValType {
    match expr {
        Value(v) => v,
        Var(x) => env.borrow().lookup(&x).clone(),
        Make(box x, box e) => {
            if let Value(ValType::Str(x)) = x {
                let val = interp_exp(e, Rc::clone(&env));
                env.borrow_mut().bind(x, val);
                ValType::Int(0)
            } else {
                panic!("Make error, variable not a literal");
            }
        },
        Print(box data) => {
            let val = interp_exp(data, Rc::clone(&env));
            match val {
                ValType::Int(n) => println!("{}", n),
                ValType::Str(str) => println!("{}", str)
            }
            ValType::Int(0)
        },
        Thing(box data) => {
            if let ValType::Str(v) = interp_exp(data, Rc::clone(&env)) {
                env.borrow().lookup(&v).clone()
            } else {
                panic!("Thing error, variable illegal");
            }
        },
        Calc(op, box n1, box n2) => {
            if let (Value(ValType::Int(v1)), Value(ValType::Int(v2))) = (n1, n2) {
                match op.as_str() {
                    "add" => ValType::Int(v1 + v2),
                    "sub" => ValType::Int(v1 - v2),
                    "mul" => ValType::Int(v1 * v2),
                    "div" => ValType::Int(v1 / v2),
                    "mod" => ValType::Int(v1 % v2),
                    _ => panic!("Calc error, illegal operator")
                }
            } else {
                panic!("Calc error, illegal variables");
            }
        }
    }
}