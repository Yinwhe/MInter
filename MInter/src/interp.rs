/*
 * @Author: Yinwhe
 * @Date: 2021-10-10 19:45:12
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-11-20 22:39:18
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

pub use crate::syntax::*;

use crate::Input;
use ansi_term::Color;
use ordered_float::OrderedFloat;
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

    let mut res = ValType::Null;
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
    ValType::Null
}

pub fn interp_exp(
    input: &mut std::io::Lines<Input<'_>>,
    expr: Expr,
    env: Rc<RefCell<SymTable<String, ValType>>>,
) -> ValType {
    use crate::parser::is_num;

    match expr {
        Value(v) => v,
        Var(x) => env.borrow().lookup(&x, true),
        Make(box x, box e) => {
            if let ValType::Str(x) = interp_exp(input, x, Rc::clone(&env)) {
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
                interp_error("Make error, variable not a literal")
            }
        }
        Erase(box n) => {
            if let Value(ValType::Str(n)) = n {
                env.borrow_mut().unbind(n);
                ValType::Num(0.0.into())
            } else {
                interp_error("Erase error, variable not a literal")
            }
        }
        Print(box data) => {
            let val = interp_exp(input, data, Rc::clone(&env));
            println!("{}", val);
            val
        }
        Thing(box data) => {
            if let ValType::Str(v) = interp_exp(input, data, Rc::clone(&env)) {
                env.borrow().lookup(&v, true)
            } else {
                interp_error("Thing error, illegal variable")
            }
        }
        Run(box cmd) => {
            if let ValType::List(list, _) = interp_exp(input, cmd, Rc::clone(&env)) {
                let mut input = Input::string(list.trim_matches(|c| c == '[' || c == ']')).lines();

                interpretor(&mut input, Rc::clone(&env))
            } else {
                interp_error("Run error, illegal cmd list")
            }
        }
        Judge(op, box value) => {
            let val = interp_exp(input, value, Rc::clone(&env));
            match op.as_str() {
                "isname" => ValType::Boolean(env.borrow().exist(&val.into())),
                "isnumber" => ValType::Boolean(is_num(val.to_string().as_str())),
                "isword" => ValType::Boolean(val.is_string()),
                "islist" => ValType::Boolean(val.is_list()),
                "isbool" => ValType::Boolean(val.is_bool()),
                "isempty" => ValType::Boolean(
                    val.to_string()
                        .trim_matches(|c| c == '[' || c == ']')
                        .is_empty(),
                ),
                _ => interp_error("Judge error, illegal operator"),
            }
        }
        Calc(op, box n1, box n2) => {
            let v1: OrderedFloat<f64> = interp_exp(input, n1, Rc::clone(&env)).into();
            let v2: OrderedFloat<f64> = interp_exp(input, n2, Rc::clone(&env)).into();
            match op.as_str() {
                "add" => ValType::Num(v1 + v2),
                "sub" => ValType::Num(v1 - v2),
                "mul" => ValType::Num(v1 * v2),
                "div" => ValType::Num(v1 / v2),
                "mod" => ValType::Num(v1 % v2),
                _ => interp_error("Calc error, illegal operator"),
            }
        }
        Comp(op, box n1, box n2) => {
            let v1 = interp_exp(input, n1, Rc::clone(&env));
            let v2 = interp_exp(input, n2, Rc::clone(&env));
            if v1.is_num() && v2.is_num() {
                // Integer compare
                let v1: OrderedFloat<f64> = v1.into();
                let v2: OrderedFloat<f64> = v2.into();
                match op.as_str() {
                    "eq" => ValType::Boolean(v1 == v2),
                    "gt" => ValType::Boolean(v1 > v2),
                    "lt" => ValType::Boolean(v1 < v2),
                    _ => interp_error("Comp error, illegal operator"),
                }
            } else {
                // String compare
                let v1: String = v1.into();
                let v2: String = v2.into();
                match op.as_str() {
                    "eq" => ValType::Boolean(v1 == v2),
                    "gt" => ValType::Boolean(v1 > v2),
                    "lt" => ValType::Boolean(v1 < v2),
                    _ => interp_error("Comp error, illegal operator"),
                }
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
                    _ => interp_error("Logic error, illegal operator"),
                }
            } else {
                interp_error("Logic error, not boolean input")
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
                interp_error("If error, condition not boolean")
            }
        }
        Read() => {
            if let Some(Ok(str)) = input.next() {
                ValType::Str(str)
            } else {
                interp_error("Read error")
            }
        }
        Return(box expr) => Retv(Box::new(interp_exp(input, expr, Rc::clone(&env)))),
        Export(box expr) => {
            if let ValType::Str(s) = interp_exp(input, expr, Rc::clone(&env)) {
                env.borrow_mut().export(s);
            } else {
                interp_error("Export error, illegal variables");
            }

            ValType::Null
        }

        Function(op, exprs) => {
            let cenv = Rc::new(RefCell::new(SymTable::new(
                Some(env.borrow().get_global()),
                None,
            )));

            let func: ValType;
            {
                func = env.borrow().lookup_global(&op);
            }

            if let ValType::List(_, ListType::Function(func_params, func_body)) = func {
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

                let mut cinput = Input::string(&func_body).lines();

                interpretor(&mut cinput, Rc::clone(&cenv))
            } else {
                interp_error("Function error, no function found")
            }
        }
        Nop => ValType::Null,
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
