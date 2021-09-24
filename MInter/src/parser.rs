/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:23:44
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-09-24 11:44:32
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

#[derive(Debug, Eq, PartialEq)]
pub enum Sexpr {
    Atom(String),
    List(Vec<Sexpr>),
}

use crate::syntax::*;
pub use Sexpr::{Atom, List};

pub fn scan(expr: &str) -> Sexpr {
    let mut stack = vec![]; // 存放 List 的栈
    let mut sym = String::new(); // 解析当前的 Atom
    let mut list = vec![]; // 解析当前的 List
    for c in expr.chars() {
        // 按字符遍历字符串
        match c {
            '(' => {
                // 新的 List 开始了
                stack.push(list); // 把当前的 list 保存到栈上
                list = vec![]; // 新建一个 list
            }
            '0'..='9' => sym.push(c), // 数字、运算符和字母都是合法的字符，加到当前的 Atom 上
            '+' | '-' | '*' | '/' => sym.push(c),
            'a'..='z' | 'A'..='Z' => sym.push(c),
            ' ' => {
                if !sym.is_empty() {
                    // 遇到空格了，如果当前正在解析 Atom，则意味着 Atom 解析完成了
                    list.push(Atom(sym)); // 将 Atom 存入列表
                    sym = String::new(); // 新建一个 Atom
                }
            }
            ')' => {
                // 当前的 List 结束了
                if !sym.is_empty() {
                    // 如果有 Atom 未存入
                    list.push(Atom(sym)); // 则存入
                    sym = String::new(); // 新建一个 Atom
                }
                let mut nlist = stack.pop().unwrap(); // 将上一个 list 出栈
                nlist.push(List(list)); // 当前的 list 作为值存入
                list = nlist; // 将上一个 list 作为当前的 list
            }
            _ => (), // 忽略其他字符
        }
    }
    if !sym.is_empty() {
        // 如果输入仅仅是一个原子，那么 sym 就非空
        return Atom(sym);
    }
    return list.pop().unwrap(); // 否则，输入是一个列表
}

pub fn parse_sexpr(sexpr: &Sexpr) -> Expr {
    match sexpr {
        Atom(s) => {
            let val: i64 = s.parse().expect("Not an integer!");
            Int(val)
        }
        List(v) => match v.as_slice() {
            [Atom(op)] if op.as_str() == "read" => Prim0(op.to_string()),
            [Atom(op), e] if op.as_str() == "-" => Prim1(op.to_string(), Box::new(parse_sexpr(e))),
            [Atom(op), e1, e2] if op.as_str() == "+" => Prim2(
                op.to_string(),
                Box::new(parse_sexpr(e1)),
                Box::new(parse_sexpr(e2)),
            ),
            _ => panic!("Invalid form!"),
        },
    }
}

pub fn parse(expr: &str) -> Expr {
    let sexpr = scan(expr);
    let expr = parse_sexpr(&sexpr);
    return expr;
}