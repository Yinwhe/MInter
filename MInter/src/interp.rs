/*
 * @Author: Yinwhe
 * @Date: 2021-10-10 19:45:12
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-12-16 00:27:12
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

pub use crate::syntax::*;

use crate::Input;
use ansi_term::Color;
use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::process::exit;
use std::rc::Rc;

pub fn interpretor(input: &mut Input, env: Rc<RefCell<SymTable<String, ValType>>>) -> ValType {
    use crate::parser::parse;

    let mut res = ValType::Null;
    while let Some(expr) = parse(input, Rc::clone(&env)) {
        res = interp_exp(input, expr, Rc::clone(&env));
        if res.is_ret_value() {
            return res.get_ret_value();
        }
    }
    res
}

fn interp_error(content: &str) -> ValType {
    println!("{} - {}", Color::Red.paint("Interpret Error"), content);
    ValType::Null
}

pub fn interp_exp(
    input: &mut Input,
    expr: Expr,
    env: Rc<RefCell<SymTable<String, ValType>>>,
) -> ValType {
    use crate::parser::is_num;

    match expr {
        Value(mut val) => {
            if let ValType::List(_, ListType::Function(closenv, _, body)) = &mut val {
                println!("Debug - It's Func!\n body: {}\n", vec2str(body));
                let mut set = HashSet::new();
                body.iter().map(|v| v.find_val_in_list(&mut set)).count();
                println!("Debug - set content: {:?}", set);
                set.iter()
                    .filter(|v| env.borrow().exist_local(v))
                    .map(|v| {
                        closenv.push(ClosureEnv {
                            name: v.to_string(),
                            val: env.borrow().lookup_local(v).unwrap(),
                        })
                    })
                    .count();
                println!("Debug - closenv: {:?}", closenv);
            }
            val
        }
        Var(x) => env.borrow().lookup(&x),
        Make(box x, box e) => {
            if let ValType::Str(x) = interp_exp(input, x, Rc::clone(&env)) {
                let mut val = interp_exp(input, e, Rc::clone(&env));
                // println!("Debug - {:?}", val);

                if let ValType::List(_, ListType::Function(_, params, _)) = &mut val {
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
            println!("{:?}", val);
            val
        }
        Thing(box data) => {
            if let ValType::Str(v) = interp_exp(input, data, Rc::clone(&env)) {
                env.borrow().lookup(&v)
            } else {
                interp_error("Thing error, illegal variable")
            }
        }
        Run(box cmd) => {
            if let ValType::List(list, _) = interp_exp(input, cmd, Rc::clone(&env)) {
                let content = vec2str(&list);
                let mut input = Input::string(&content.trim_matches(|c| c == '[' || c == ']'));

                interpretor(&mut input, Rc::clone(&env))
            } else {
                interp_error("Run error, illegal cmd list")
            }
        }
        Judge(op, box value) => {
            let val = interp_exp(input, value, Rc::clone(&env));
            match op.as_str() {
                "isname" => ValType::Boolean(env.borrow().exist_local(&val.to_string())),
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
                let v1 = v1.to_string();
                let v2 = v2.to_string();
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
            if let Some(str) = input.next_word() {
                ValType::Str(str)
            } else {
                interp_error("Read error")
            }
        }
        Return(box expr) => {
            let res = Retv(Box::new(interp_exp(input, expr, Rc::clone(&env))));
            println!("Debug - Return val: {:?}", res);
            res
        }
        Export(box expr) => {
            if let ValType::Str(s) = interp_exp(input, expr, Rc::clone(&env)) {
                env.borrow_mut().export(s);
            } else {
                interp_error("Export error, illegal variables");
            }

            ValType::Null
        }

        Function(op, exprs) => {
            // println!("Debug - run func: {}", op);
            let cenv = Rc::new(RefCell::new(SymTable::new(
                Some(env.borrow().get_global()),
                None,
            )));

            let func: ValType;
            {
                func = env.borrow().lookup_local(&op).unwrap();
            }

            if let ValType::List(_, ListType::Function(closenv, func_params, func_body)) = func {
                let mut params = VecDeque::new();
                let func_body = vec2str(&func_body);
                exprs
                    .into_iter()
                    .map(|expr| params.push_back(interp_exp(input, expr, Rc::clone(&env))))
                    .count();
                closenv
                    .into_iter()
                    .map(|c| cenv.borrow_mut().bind(c.name, c.val))
                    .count();
                func_params
                    .into_iter()
                    .map(|param_name| {
                        cenv.borrow_mut()
                            .bind(param_name, params.pop_front().unwrap())
                    })
                    .count();

                let mut cinput = Input::string(&func_body.trim_matches(|c| c == '[' || c == ']'));

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
