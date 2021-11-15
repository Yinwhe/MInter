/*
 * @Author: Yinwhe
 * @Date: 2021-10-10 19:45:12
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-10-12 20:05:07
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

use crate::parser::{is_digit, parse};
pub use crate::syntax::Expr::{self, *};
pub use crate::syntax::*;
use crate::Input;
use std::cell::RefCell;
use std::io::BufRead;
use std::rc::Rc;

pub fn interp_exp(
    input: &mut std::io::Lines<Input<'_>>,
    expr: Expr,
    env: Rc<RefCell<SymTable<String, ValType>>>,
) -> ValType {
    match expr {
        Value(v) => v,
        Var(x) => env.borrow().lookup(&x).clone(),
        Make(box x, box e) => {
            if let Value(ValType::Str(x)) = x {
                let val = interp_exp(input, e, Rc::clone(&env));
                env.borrow_mut().bind(x, val);
                ValType::Int(0)
            } else {
                panic!("Make error, variable not a literal");
            }
        }
        Erase(box n) => {
            if let Value(ValType::Str(n)) = n {
                env.borrow_mut().unbind(n);
                ValType::Int(0)
            } else {
                panic!("Erase error, variable not a literal");
            }
        }
        Print(box data) => {
            let val = interp_exp(input, data, Rc::clone(&env));
            match &val {
                ValType::Int(n) => {
                    println!("{}", n);
                }
                ValType::Str(str) => {
                    println!("{}", str);
                }
                ValType::Boolean(b) => {
                    println!("{}", b);
                }
                ValType::List(_list) => {
                    unreachable!() // Not supported yet
                }
            }
            val
        }
        Thing(box data) => {
            if let ValType::Str(v) = interp_exp(input, data, Rc::clone(&env)) {
                env.borrow().lookup(&v).clone()
            } else {
                panic!("Thing error, illegal variable");
            }
        }
        Run(box cmd) => {
            if let ValType::Str(c) = interp_exp(input, cmd, Rc::clone(&env)) {
                let mut input = Input::string(c.trim_matches(|c| c == '[' || c == ']')).lines();
                let exps = parse(&mut input);
                exps.into_iter()
                    .map(|exp| interp_exp(&mut input, exp, Rc::clone(&env)))
                    .last()
                    .unwrap()
            } else {
                panic!("Run error, illegal cmd list")
            }
        }
        Judge(op, box value) => {
            let val: String = interp_exp(input, value, Rc::clone(&env)).into();
            match op.as_str() {
                "isname" => ValType::Boolean(env.borrow().exist(&val)),
                "isnumber" => ValType::Boolean(is_digit(&val)),
                _ => panic!("Judge error, illegal operator"),
            }
        }
        Calc(op, box n1, box n2) => {
            let v1: i64 = interp_exp(input, n1, Rc::clone(&env)).into();
            let v2: i64 = interp_exp(input, n2, Rc::clone(&env)).into();
            match op.as_str() {
                "add" => ValType::Str((v1 as f64 + v2 as f64).to_string()),
                "sub" => ValType::Str((v1 as f64 - v2 as f64).to_string()),
                "mul" => ValType::Str((v1 as f64 * v2 as f64).to_string()),
                "div" => ValType::Str((v1 as f64 / v2 as f64).to_string()),
                "mod" => ValType::Int(v1 % v2),
                _ => panic!("Calc error, illegal operator"),
            }
        }
        Comp(op, box n1, box n2) => {
            let v1: i64 = interp_exp(input, n1, Rc::clone(&env)).into();
            let v2: i64 = interp_exp(input, n2, Rc::clone(&env)).into();
            match op.as_str() {
                "eq" => ValType::Boolean(v1 == v2),
                "gt" => ValType::Boolean(v1 > v2),
                "lt" => ValType::Boolean(v1 < v2),
                _ => panic!("Comp error, illegal operator"),
            }
        }
        Logic(op, box n1, box n2) => {
            if let (ValType::Boolean(b1), ValType::Boolean(b2)) = (
                interp_exp(input, n1, Rc::clone(&env)),
                interp_exp(input, n2, Rc::clone(&env)),
            ) {
                match op.as_str() {
                    "and" => ValType::Boolean(b1 && b2),
                    "or" => ValType::Boolean(b1 || b2),
                    "not" => ValType::Boolean(!b1),
                    _ => panic!("Logic error, illegal operator"),
                }
            } else {
                panic!("Logic error, not boolean input")
            }
        }
        If(box b, r1, r2) => {
            if let ValType::Boolean(b) = interp_exp(input, b, Rc::clone(&env)) {
                if b {
                    interp_exp(input, Run(r1), Rc::clone(&env))
                } else {
                    interp_exp(input, Run(r2), Rc::clone(&env))
                }
            } else {
                panic!("If error, condition not boolean")
            }
        }
        Read() => {
            if let Some(Ok(str)) = input.next() {
                ValType::Str(str)
            } else {
                panic!("Read error");
            }
        }
    }
}
