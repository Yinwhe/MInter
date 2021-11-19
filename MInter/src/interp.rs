/*
 * @Author: Yinwhe
 * @Date: 2021-10-10 19:45:12
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-11-19 12:09:56
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

use ansi_term::Color;

pub use crate::syntax::*;
use crate::Input;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::BufRead;
use std::process::exit;
use std::rc::Rc;

pub fn interpretor(
    input: &mut std::io::Lines<Input<'_>>,
    env: Rc<RefCell<SymTable<String, ValType>>>,
) -> ValType {
    use crate::parser::parse;

    // let exps = parse(input);
    // if exps.is_empty() {
    //     return ValType::Retv(Box::new(ValType::Int(0)));
    // }
    let mut res = ValType::ErrorValue;
    for exp in parse(input) {
        res = interp_exp(input, exp, Rc::clone(&env));
        if res.is_ret_value() {
            return res;
        }
    }
    res
}

fn interp_error(content: &str) -> ValType {
    println!("{} - {}", Color::Red.paint("Interpret Error"), content);
    ValType::ErrorValue
}

pub fn interp_exp(
    input: &mut std::io::Lines<Input<'_>>,
    expr: Expr,
    env: Rc<RefCell<SymTable<String, ValType>>>,
) -> ValType {
    use crate::parser::is_digit;

    match expr {
        Value(v) => v,
        Var(x) => env.borrow().lookup_with_env(&x),
        Make(box x, box e) => {
            if let Value(ValType::Str(x)) = x {
                let val = interp_exp(input, e, Rc::clone(&env));

                if let ValType::List(_, ListType::Function(params, _)) = &val {
                    FUNC_NAME
                        .lock()
                        .unwrap()
                        .insert(x.clone(), params.len() as i32);
                }

                env.borrow_mut().bind(x, val.clone());
                val
            } else {
                interp_error("Make interp_error, variable not a literal")
            }
        }
        Erase(box n) => {
            if let Value(ValType::Str(n)) = n {
                env.borrow_mut().unbind(n);
                ValType::Int(0)
            } else {
                interp_error("Erase interp_error, variable not a literal")
            }
        }
        Print(box data) => {
            let val = interp_exp(input, data, Rc::clone(&env));
            println!("{}", val);
            val
        }
        Thing(box data) => {
            if let ValType::Str(v) = interp_exp(input, data, Rc::clone(&env)) {
                env.borrow().lookup_with_env(&v)
            } else {
                interp_error("Thing interp_error, illegal variable")
            }
        }
        Run(box cmd) => {
            if let ValType::List(list, _) = interp_exp(input, cmd, Rc::clone(&env)) {
                let mut input = Input::string(list.trim_matches(|c| c == '[' || c == ']')).lines();

                interpretor(&mut input, Rc::clone(&env))
            } else {
                interp_error("Run interp_error, illegal cmd list")
            }
        }
        Judge(op, box value) => {
            let val: String = interp_exp(input, value, Rc::clone(&env)).into();
            match op.as_str() {
                "isname" => ValType::Boolean(env.borrow().exist(&val)),
                "isnumber" => ValType::Boolean(is_digit(&val)),
                _ => interp_error("Judge interp_error, illegal operator"),
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
                _ => interp_error("Calc interp_error, illegal operator"),
            }
        }
        Comp(op, box n1, box n2) => {
            let v1: i64 = interp_exp(input, n1, Rc::clone(&env)).into();
            let v2: i64 = interp_exp(input, n2, Rc::clone(&env)).into();
            match op.as_str() {
                "eq" => ValType::Boolean(v1 == v2),
                "gt" => ValType::Boolean(v1 > v2),
                "lt" => ValType::Boolean(v1 < v2),
                _ => interp_error("Comp interp_error, illegal operator"),
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
                    _ => interp_error("Logic interp_error, illegal operator"),
                }
            } else {
                interp_error("Logic interp_error, not boolean input")
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
                interp_error("If interp_error, condition not boolean")
            }
        }
        Read() => {
            if let Some(Ok(str)) = input.next() {
                ValType::Str(str)
            } else {
                interp_error("Read interp_error")
            }
        }
        Return(box expr) => Retv(Box::new(interp_exp(input, expr, Rc::clone(&env)))),

        Function(op, exprs) => {
            let cenv = Rc::new(RefCell::new(SymTable::new(Some(Rc::clone(&env)))));
            
            if let ValType::List(_, ListType::Function(func_params, func_body)) =
                env.borrow().lookup_with_env(&op)
            {
                let mut params = VecDeque::new();
                exprs
                    .into_iter()
                    .map(|expr| params.push_back(interp_exp(input, expr, Rc::clone(&env))))
                    .count();

                func_params
                    .to_owned()
                    .into_iter()
                    .map(|param_name| {
                        cenv.borrow_mut()
                            .bind(param_name, params.pop_front().unwrap())
                    })
                    .count();

                let mut input = Input::string(&func_body).lines();

                interpretor(&mut input, Rc::clone(&cenv))
            } else {
                interp_error("Function interp_error, no function found")
            }
        }
        ErrorExpr => ValType::ErrorValue,
        Exit => {
            println!(
                "{}",
                Color::RGB(0x33, 0xff, 0xcc).paint(
                    "
            ____
            | __ ) _   _  ___
            |  _ \\| | | |/ _ \\
            | |_) | |_| |  __/
            |____/ \\__, |\\___|
                   |___/
            "
                )
            );
            exit(0);
        }
    }
}
