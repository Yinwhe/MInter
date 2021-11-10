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

use crate::syntax::*;
use crate::Input;
pub use Sexpr::{Atom, List};

fn is_valid_op(op: &str) -> Option<&i32> {
    return VALID_OP.get(&op);
}

// Read one whole command a time
pub fn parse_list(input: &mut std::io::Lines<Input<'_>>) -> Sexpr {
    let mut stack = vec![];
    let mut list = vec![];

    let mut param_num = 0;
    let mut param_stack = vec![];

    let mut literal = String::new(); // String to store a list
    let mut braket_num = 0; // Used to read list.

    while let Some(Ok(expr)) = input.next() {
        for word in expr.split_whitespace() {
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
                // Check list first
                if word.starts_with("[") {
                    braket_num += 1;
                    if braket_num == 1 {
                        literal.clear();
                    }
                }

                if word.ends_with("]") {
                    braket_num -= 1;
                    if braket_num == 0 {
                        literal.extend([word, " "]);
                        braket_num = -1; // Act as a flag
                    }
                }

                if braket_num > 0 {
                    literal.extend([word, " "]);
                } else {
                    if braket_num == -1 {
                        list.push(Atom(literal.trim().into()));
                        braket_num = 0; // Clear flag
                    } else {
                        list.push(Atom(word.into()));
                    }

                    param_num -= 1;
                    if param_num <= 0 {
                        let mut nlist = stack.pop().unwrap();
                        param_num = param_stack.pop().unwrap();
                        if param_num == 0 {
                            break; // Jump out of for_loop
                        }
                        nlist.push(List(list));
                        param_num -= 1;
                        list = nlist;
                    }
                }
            }
        }

        if param_num == 0 && param_stack.is_empty() {
            break; // Already read in a command, jump out of loop
        }
    }
    // println!("debug - {:?}", list);
    List(list)
}

pub fn is_digit(s: &str) -> bool {
    // Here we won't implement float number
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

pub fn is_list(s: &str) -> bool {
    return s.starts_with("[");
}

pub fn parse_sexpr(sexpr: &Sexpr) -> Expr {
    match sexpr {
        Atom(s) => {
            if is_digit(s) {
                return Value(ValType::Int(s.parse().unwrap()));
            } else if is_literal(s) {
                return Value(ValType::Str(s[1..].to_string()));
            } else if is_list(s) {
                return Value(ValType::Str(s.to_string()));
            } else if is_var(s) {
                return Var(s[1..].to_string());
            } else {
                panic!("Unregconized Atom");
            }
        }
        List(v) => match v.as_slice() {
            // 2 parameters
            [Atom(op), param1, param2] => match op.as_str() {
                "make" => Make(Box::new(parse_sexpr(param1)), Box::new(parse_sexpr(param2))),
                "add" | "sub" | "mul" | "div" | "mod" => Calc(
                    op.to_string(),
                    Box::new(parse_sexpr(param1)),
                    Box::new(parse_sexpr(param2)),
                ),
                _ => {
                    panic!("Unrecognized List 2");
                }
            },
            // 1 parameters
            [Atom(op), param] => match op.as_str() {
                "print" => Print(Box::new(parse_sexpr(param))),
                "thing" => Thing(Box::new(parse_sexpr(param))),
                "erase" => Erase(Box::new(parse_sexpr(param))),
                "run"   => Run(Box::new(parse_sexpr(param))),
                _ => {
                    panic!("Unrecognized List 1");
                }
            },
            // no parameters
            [Atom(op)] => match op.as_str() {
                "read" => Read(),
                _ => {
                    panic!("Unrecognized List 0");
                }
            },
            _ => panic!("Invalid syntax!"),
        },
    }
}

pub fn parse(input: &mut std::io::Lines<Input<'_>>) -> Expr {
    let sexpr = parse_list(input);
    let expr = parse_sexpr(&sexpr);
    return expr;
}
