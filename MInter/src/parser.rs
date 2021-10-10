/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:23:44
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-10-10 20:06:45
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

const valid_op: [&'static str; 2] = ["make", "print"];

fn is_valid_op(op: &str) -> bool {
    return valid_op.contains(&op);
}

pub fn parse_list(expr: &str) -> Sexpr {
    let mut stack = vec![];
    let mut list = vec![];

    for word in expr.split(' ') {
        if is_valid_op(word) {
            stack.push(list);
            list = vec![];
            list.push(Atom(word.into()));
        } else {
            list.push(Atom(word.into()));
        }
    }

    while let Some(mut nlist) = stack.pop() {
        // The initially pushed empty vec shall be ignored
        if !nlist.is_empty() {
            nlist.push(List(list));
            list = nlist;
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
            // else if is_var(s) {
            //     return Var(s[1..].to_string());
            // }
            else {
                panic!("Unregconized Atom");
            }
        }
        List(v) => match v.as_slice() {
            // make
            [Atom(op), Atom(var), val] if op.as_str() == "make" => {
                Make(Box::new(Var(var.to_string())), Box::new(parse_sexpr(val)))
            },
            [Atom(op), data] if op.as_str() == "print" => {
                Print(Box::new(parse_sexpr(data)))
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
