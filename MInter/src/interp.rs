/*
 * @Author: Yinwhe
 * @Date: 2021-10-10 19:45:12
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-12-22 11:27:27
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

pub use crate::syntax::*;

use crate::{vecdeque, Input};
use ansi_term::Color;
use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{Write, Read};
use std::process::exit;
use std::rc::Rc;

pub fn interpretor(input: &mut Input, env: Rc<RefCell<SymTable>>) -> ValType {
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

pub fn interp_exp(input: &mut Input, expr: Expr, env: Rc<RefCell<SymTable>>) -> ValType {
    use crate::parser::is_num;

    match expr {
        Value(mut val) => {
            if let ValType::List(_, ListType::Function(closenv, _, body)) = &mut val {
                // println!("Debug - It's Func!\n body: {}\n", vec2str(body));
                let mut set = HashSet::new();
                body.iter().map(|v| v.find_val_in_list(&mut set)).count();
                // println!("Debug - set content: {:?}", set);
                set.iter()
                    .filter(|v| env.borrow().exist_local(v))
                    .map(|v| {
                        closenv.push(ClosureEnv {
                            name: v.to_string(),
                            val: env.borrow().lookup_local(v).unwrap(),
                        })
                    })
                    .count();
                // println!("Debug - closenv: {:?}", closenv);
            }
            val
        }
        Var(x) => env.borrow().lookup(&x).unwrap_or(ValType::Null),
        Make(box x, box e) => {
            if let ValType::Str(x) = interp_exp(input, x, Rc::clone(&env)) {
                let val = interp_exp(input, e, Rc::clone(&env));
                // println!("Debug - {:?}", val);

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
                env.borrow().lookup(&v).unwrap_or(ValType::Null)
            } else {
                interp_error("Thing error, illegal variable")
            }
        }
        Run(box cmd) => {
            if let ValType::List(list, _) = interp_exp(input, cmd, Rc::clone(&env)) {
                let content = vec2str(&list);
                let mut input = Input::string(&content[1..content.len() - 1]);

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
        Index(op, box value) => {
            let list = interp_exp(input, value, Rc::clone(&env));
            if let ValType::List(mut list, _) = list {
                match op.as_str() {
                    "first" => list.pop_front().unwrap(),
                    "last" => list.pop_back().unwrap(),
                    "butfirst" => {
                        list.pop_front();
                        ValType::List(list, ListType::Ordinary)
                    }
                    "butlast" => {
                        list.pop_back();
                        ValType::List(list, ListType::Ordinary)
                    }
                    _ => interp_error("Index error, illegal operator"),
                }
            } else {
                let str = list.to_string();
                match op.as_str() {
                    "first" => ValType::Str(str.as_bytes().first().unwrap().to_string()),
                    "last" => ValType::Str(str.as_bytes().last().unwrap().to_string()),
                    "butfirst" => ValType::Str(str[1..].to_owned()),
                    "butlast" => ValType::Str(str[0..str.len() - 1].to_owned()),
                    _ => interp_error("Index error, illegal operator"),
                }
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
        Extend(op, box l1, box l2) => {
            let v1 = interp_exp(input, l1, Rc::clone(&env));
            let v2 = interp_exp(input, l2, Rc::clone(&env));
            match op.as_str() {
                "sentence" => {
                    let mut list = v1.to_list();
                    list.extend(v2.to_list());
                    ValType::List(list, ListType::Ordinary)
                }
                "list" => ValType::List(vecdeque![v1, v2], ListType::Ordinary),
                "join" => {
                    let mut list = v1.to_list();
                    list.push_back(v2);
                    ValType::List(list, ListType::Ordinary)
                }
                _ => interp_error("Extend error, illegal operator"),
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
        Read => {
            if let Some(str) = input.next_word() {
                ValType::Str(str)
            } else {
                interp_error("Read error")
            }
        }
        Return(box expr) => {
            let res = Retv(Box::new(interp_exp(input, expr, Rc::clone(&env))));
            // println!("Debug - Return val: {:?}", res);
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
                func = env.borrow().lookup(&op).unwrap();
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

                let mut cinput = Input::string(&func_body[1..func_body.len() - 1]);

                interpretor(&mut cinput, Rc::clone(&cenv))
            } else {
                interp_error("Function error, no function found")
            }
        }
        Save(box filename) => {
            let filename = interp_exp(input, filename, Rc::clone(&env)).to_string();
            let mut file = File::create(&filename).unwrap();

            for (key, val) in env.borrow().get_keys_values() {
                let str = format!("make \"{} {}\n", key, val.to_origin());
                file.write_all(str.as_bytes()).unwrap();
            }
            ValType::Str(filename)
        }
        Load(box filename) => {
            let filename = interp_exp(input, filename, Rc::clone(&env)).to_string();
            let mut file = File::open(&filename).unwrap();

            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let mut input = Input::string(&content);
            interpretor(&mut input, Rc::clone(&env));

            ValType::Boolean(true)
        }
        Erall => {
            env.borrow_mut().clear_all();
            ValType::Boolean(true)
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
