/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:23:44
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-12-16 00:46:35
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

pub use Sexpr::{Atom, List};

use crate::syntax::*;
use crate::Input;
use ansi_term::Color;
use regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
pub enum Sexpr {
    Atom(String),
    List(Vec<Sexpr>),
}

fn is_valid_op(key: &String, env: Rc<RefCell<SymTable>>) -> Option<i32> {
    if let Some(&n) = KEYWORD.get(key.as_str()) {
        Some(n)
    } else {
        env.borrow().is_func(key)
    }
}

fn is_func(sexpr: Option<&Sexpr>, env: Rc<RefCell<SymTable>>) -> Option<String> {
    if let Some(Atom(op)) = sexpr {
        env.borrow().is_func(op).map(|_| op.to_owned())
    } else {
        None
    }
}

fn parse_error(content: &str) -> Expr {
    println!("{} - {}", Color::Red.paint("Parse Error"), content);
    Expr::Nop
}

// Read until a command line is complete
pub fn parse_string(
    input: &mut Input,
    env: Rc<RefCell<SymTable>>,
) -> Option<Sexpr> {
    let mut stack = vec![];
    let mut list = vec![];

    let mut param_num = 0;
    let mut param_stack = vec![];

    let mut literal = String::new(); // String to store a list
    let mut braket_num = 0; // Used to read list.

    let mut atom: bool;
    let mut valid_op: bool;

    while let Some(word) = &input.next_word() {
        if let Some(n) = is_valid_op(word, Rc::clone(&env)) {
            valid_op = true;

            if braket_num > 0 {
                // Words in list are atoms
                atom = true;
            } else {
                // When needed parameter's number is zero
                // the op shall be taken as an atom
                atom = n <= 0;

                param_stack.push(param_num);
                param_num = n;
                stack.push(list);
                list = vec![];
            }
        } else {
            valid_op = false;
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

                if param_num == 0 && !valid_op {
                    // Value input
                    continue;
                } else if param_num != 0 {
                    param_num -= 1;
                }

                while param_num <= 0 {
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
        if param_stack.is_empty() {
            break; // Jump out of the loop
        }
    } // While
    println!("Debug - parse string res: {:?}", list);
    list.pop()
}

pub fn is_num(s: &str) -> bool {
    let mut x = s;
    if s.starts_with("-") {
        x = &s[1..];
    }
    !x.is_empty() && x.chars().find(|&c| !(c.is_digit(10) || c == '.')).is_none()
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

fn parse_list(slist: &str) -> Vec<ValType> {
    let mut stack = vec![];
    let mut list = vec![];
    let mut word = "".to_string();
    let mut word_flag = false;

    for c in slist.chars() {
        match c {
            '[' => {
                stack.push(list);
                list = vec![];
            }
            ']' => {
                if word_flag {
                    word_flag = false;
                    list.push(ValType::Str(word.clone()));
                    word.clear();
                }
                let mut nlist = stack.pop().unwrap();
                nlist.push(ValType::List(list, ListType::Ordinary));
                list = nlist;
            }
            ' ' => {
                if word_flag {
                    word_flag = false;
                    list.push(ValType::Str(word.clone()));
                    word.clear();
                }
            }
            _ => {
                word_flag = true;
                word.push(c);
            }
        }
    }

    if let ValType::List(l, _) = list.pop().unwrap() {
        l
    } else {
        panic!("parse list fatal error");
    }
}

fn solve_list(s: &str) -> ValType {
    let re = Regex::new(r"^\[ ?\[([^\]]*)\] \[(.*)\] ?\]$").unwrap();

    if let Some(m) = re.captures(s) {
        ValType::List(
            parse_list(s),
            ListType::Function(
                Vec::new(),
                m.get(1)
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
                parse_list(&format!("[{}]", m.get(2).unwrap().as_str())),
            ),
        )
    } else {
        ValType::List(parse_list(s), ListType::Ordinary)
    }
}

pub fn parse_sexpr(sexpr: &Sexpr) -> Expr {
    match sexpr {
        Atom(s) => {
            if is_num(s) {
                Value(ValType::Num(s.parse().unwrap()))
            } else if is_literal(s) {
                Value(ValType::Str(s[1..].to_string()))
            } else if is_list(s) {
                Value(solve_list(s))
            } else if is_var(s) {
                Var(s[1..].to_string())
            } else {
                parse_error(&format!("Unregconized Atom {}", s))
            }
        }
        List(v) => {
            if let Some(func_name) = is_func(v.first(), en) {
                // Function
                Function(
                    func_name,
                    v.iter().skip(1).map(|sexpr| parse_sexpr(sexpr)).collect(),
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
                        _ => parse_error("Unrecognized List 3"),
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
                        _ => parse_error("Unrecognized List 2"),
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
                        "return" => Return(Box::new(parse_sexpr(param))),
                        "export" => Export(Box::new(parse_sexpr(param))),
                        _ => parse_error("Unrecognized List 1"),
                    },
                    // no parameters
                    [Atom(op)] => match op.as_str() {
                        "nop" => Nop,
                        "read" => Read(),
                        "exit" => Exit,
                        _ => parse_error("Unrecognized List 0"),
                    },
                    _ => parse_error("Invalid syntax!"),
                }
            }
        }
    }
}

pub fn parse(input: &mut Input, env: Rc<RefCell<SymTable>>) -> Option<Expr> {
    parse_string(input, Rc::clone(&env)).map(|sexpr| parse_sexpr(&sexpr))
}
