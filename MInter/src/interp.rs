/*
 * @Author: Yinwhe
 * @Date: 2021-10-10 19:45:12
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-10-12 20:05:07
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
        }
        Print(box data) => {
            let val = interp_exp(data, Rc::clone(&env));
            match val {
                ValType::Int(n) => {
                    println!("{}", n);
                    ValType::Int(n.to_string().len() as i64)
                },
                ValType::Str(str) => {
                    println!("{}", str);
                    ValType::Int(str.len() as i64)
                },
            }
        }
        Thing(box data) => {
            if let ValType::Str(v) = interp_exp(data, Rc::clone(&env)) {
                env.borrow().lookup(&v).clone()
            } else {
                panic!("Thing error, variable illegal");
            }
        }
        Calc(op, box n1, box n2) => {
            let v1: i64 = interp_exp(n1, Rc::clone(&env)).into();
            let v2: i64 = interp_exp(n2, Rc::clone(&env)).into();
            match op.as_str() {
                "add" => ValType::Int(v1 + v2),
                "sub" => ValType::Int(v1 - v2),
                "mul" => ValType::Int(v1 * v2),
                "div" => ValType::Int(v1 / v2),
                "mod" => ValType::Int(v1 % v2),
                _ => panic!("Calc error, illegal operator"),
            }
        }
        Read() => {
            let mut str = String::new();
            std::io::stdin().read_line(&mut str).unwrap();
            str.pop(); /* Remove Enter */
            ValType::Str(str)
        }
    }
}
