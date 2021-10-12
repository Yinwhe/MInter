/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:23:44
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-10-12 19:50:39
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

#[derive(Debug, Eq, PartialEq)]
pub enum Sexpr {
    Atom(String),
    List(Vec<Sexpr>),
}


pub use Sexpr::{Atom, List};
use crate::syntax::*;

fn is_valid_op(op: &str) -> Option<&i32> {
    return VALID_OP.get(&op);
}


pub fn parse_list(expr: &str) -> Sexpr {
    let mut stack = vec![];
    let mut list = vec![];
    let mut param_num = 0;
    let mut param_stack = vec![];

    for word in expr.split(' ') {
        if let Some(&n) = is_valid_op(word) {
            if n == 0 {
                list.push(List(vec![Atom(word.into())]))
            } else {
                param_stack.push(param_num);
                param_num = n;
                stack.push(list);
                list = vec![];
                list.push(Atom(word.into()));
            }
        } else {
            list.push(Atom(word.into()));
            param_num -= 1;
            if param_num <= 0 {
                let mut nlist = stack.pop().unwrap();
                param_num = param_stack.pop().unwrap();
                if param_num == 0 {
                    break;
                }
                nlist.push(List(list));
                param_num -= 1;
                list = nlist;
            }
        }
    }
    
    List(list)
}

pub fn is_digit(s: &str) -> bool {
    // Here we haven't implement judging float number
    for c in s.chars() {
        if !c.is_digit(10) {
            return false;
        }
    }
    return true;
}

pub fn is_literal(s: &str) -> bool {
    return s.starts_with("\"");
}
pub fn is_var(s: &str) -> bool {
    return s.starts_with(":");
}

pub fn parse_sexpr(sexpr: &Sexpr) -> Expr {
    match sexpr {
        Atom(s) => {
            if is_digit(s) {
                return Value(ValType::Int(s.parse().unwrap()));
            }
            else if is_literal(s) {
                return Value(ValType::Str(s[1..].to_string()));
            }
            else if is_var(s) {
                return Var(s[1..].to_string());
            }
            else {
                panic!("Unregconized Atom");
            }
        }
        List(v) => match v.as_slice() {
            // make
            [Atom(op), param1, param2] => {
                match op.as_str() {
                    "make" => {
                        Make(Box::new(parse_sexpr(param1)), Box::new(parse_sexpr(param2)))
                    },
                    "add" | "sub" | "mul" | "div" | "mod" => {
                        Calc(op.to_string(), Box::new(parse_sexpr(param1)), Box::new(parse_sexpr(param2)))
                    },
                    _ => {
                        panic!("Unrecognized List 2");
                    }
                }
            },
            [Atom(op), param] => {
                match op.as_str() {
                    "print" => {
                        Print(Box::new(parse_sexpr(param)))
                    },
                    "thing" => {
                        Thing(Box::new(parse_sexpr(param)))
                    },
                    _ => {
                        panic!("Unrecognized List 1");
                    }
                }
            },
            [Atom(op)] => {
                match op.as_str() {
                    "read" => Read(),
                    _ => {
                        panic!("Unrecognized List 0");
                    }
                }
            }
            _ => panic!("Invalid syntax!")
        }
    }
}

pub fn parse(expr: &str) -> Expr {
    let sexpr = parse_list(expr);
    let expr = parse_sexpr(&sexpr);
    return expr;
}
