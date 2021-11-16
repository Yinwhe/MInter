/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:23:44
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-11-16 21:32:29
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
use regex::Regex;
pub use Sexpr::{Atom, List};

fn is_valid_op(key: &str) -> Option<i32> {
    if let Some(&n) = KEYWORD.get(key) {
        Some(n)
    } else {
        FUNC_NAME.lock().unwrap().get(key).map(|n| n.to_owned())
    }
}

fn is_func(sexpr: Option<&Sexpr>) -> Option<String> {
    if let Some(Atom(op)) = sexpr {
        if FUNC_NAME.lock().unwrap().get(op).is_some() {
            Some(op.to_string())
        } else {
            None
        }
    } else {
        None
    }
}

// Read one whole command a time
pub fn parse_list(input: &mut std::io::Lines<Input<'_>>) -> Vec<Sexpr> {
    let mut stack = vec![];
    let mut list = vec![];

    let mut param_num = 0;
    let mut param_stack = vec![];

    let mut literal = String::new(); // String to store a list
    let mut braket_num = 0; // Used to read list.

    let mut atom: bool;

    while let Some(Ok(expr)) = input.next() {
        for word in expr.split_whitespace() {
            if let Some(n) = is_valid_op(&word) {
                // When needed parameter's number is zeor
                // the op shall be taken as an atom
                atom = n <= 0;

                param_stack.push(param_num);
                param_num = n;
                stack.push(list);
                list = vec![];
            } else {
                atom = true;
            }

            if !atom {
                list.push(Atom(word.into()));
            } else {
                // Check list first
                if word.starts_with("[") {
                    braket_num += word.matches("[").count() as i32;
                    if braket_num == 1 {
                        literal.clear();
                    }
                }

                if word.ends_with("]") {
                    braket_num -= word.matches("]").count() as i32;
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

                    if param_num == 0 {
                        // Value input
                        continue;
                    }

                    param_num -= 1;
                    while param_num == 0 {
                        let mut nlist = stack.pop().unwrap();
                        param_num = param_stack.pop().unwrap();
                        nlist.push(List(list));
                        list = nlist;
                        if param_stack.is_empty() {
                            break;
                        }
                        param_num -= 1;
                    }
                }
            }
        } // For
        if param_stack.is_empty() {
            break; // Jump out of the loop
        }
    } // While
      // println!("{:?}", list);
    list
}

pub fn is_digit(s: &str) -> bool {
    // Here we won't implement float number
    for c in s.chars() {
        if !c.is_digit(10) {
            return false;
        }
    }
    true
}

fn is_literal(s: &str) -> bool {
    s.starts_with("\"")
}
fn is_var(s: &str) -> bool {
    s.starts_with(":")
}

fn is_list(s: &str) -> bool {
    s.starts_with("[")
}

fn solve_list(s: &str) -> ValType {
    let re = Regex::new(r"^\[ \[([^\]]*)\] \[(.*)\] \]$").unwrap();

    if let Some(m) = re.captures(s) {
        ValType::List(
            s.to_string(),
            ListType::Function(
                m.get(1).unwrap().as_str().to_string(),
                // .split_whitespace()
                // .map(|s| s.to_string())
                // .collect(),
                m.get(2).unwrap().as_str().to_string(),
            ),
        )
    } else {
        ValType::List(s.to_string(), ListType::Ordinary)
    }
}

pub fn parse_sexpr(sexpr: &Sexpr) -> Expr {
    match sexpr {
        Atom(s) => {
            if is_digit(s) {
                Value(ValType::Int(s.parse().unwrap()))
            } else if is_literal(s) {
                Value(ValType::Str(s[1..].to_string()))
            } else if is_list(s) {
                Value(solve_list(s))
            } else if is_var(s) {
                Var(s[1..].to_string())
            } else {
                panic!("Unregconized Atom");
            }
        }
        List(v) => {
            if let Some(func_name) = is_func(v.first()) {
                // Function
                Function(
                    func_name,
                    v.iter().map(|sexpr| parse_sexpr(sexpr)).collect(),
                )
            } else {
                match v.as_slice() {
                    // 3 parameters
                    [Atom(op), param1, param2, param3] => match op.as_str() {
                        "if" => If(
                            Box::new(parse_sexpr(param1)),
                            Box::new(parse_sexpr(param2)),
                            Box::new(parse_sexpr(param3)),
                        ),
                        _ => panic!("Unrecognized List 3"),
                    },
                    // 2 parameters
                    [Atom(op), param1, param2] => match op.as_str() {
                        "make" => {
                            Make(Box::new(parse_sexpr(param1)), Box::new(parse_sexpr(param2)))
                        }
                        "add" | "sub" | "mul" | "div" | "mod" => Calc(
                            op.to_string(),
                            Box::new(parse_sexpr(param1)),
                            Box::new(parse_sexpr(param2)),
                        ),
                        "eq" | "gt" | "lt" => Comp(
                            op.to_string(),
                            Box::new(parse_sexpr(param1)),
                            Box::new(parse_sexpr(param2)),
                        ),
                        "and" | "or" => Logic(
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
                        "run" => Run(Box::new(parse_sexpr(param))),
                        "not" => Logic(
                            "not".to_string(),
                            Box::new(parse_sexpr(param)),
                            Box::new(Value(ValType::Boolean(true))),
                        ),
                        "isname" | "isnumber" | "isword" | "islist" | "isbool" | "isempty" => {
                            Judge(op.to_string(), Box::new(parse_sexpr(param)))
                        }
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
                }
            }
        }
    }
}

pub fn parse(input: &mut std::io::Lines<Input<'_>>) -> Vec<Expr> {
    // let sexprs = parse_list(input)
    //     .iter()
    //     .map(|sexpr| parse_sexpr(sexpr))
    //     .collect();
    // println!("sexpr - {:?}", sexprs);
    // return sexprs;

    parse_list(input)
        .iter()
        .map(|sexpr| parse_sexpr(sexpr))
        .collect()
}
