/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:12:25
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-09-24 16:53:07
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */
#![feature(box_patterns)]

mod helper;
mod parser;
mod syntax;

pub use crate::syntax::Expr::{self, *};
pub use crate::syntax::SymTable;
use std::cell::RefCell;
use std::rc::Rc;

fn interp_exp(expr: Expr, env: Rc<RefCell<SymTable<String, i64>>>) -> i64 {
    match expr {
        Int(n) => n,
        Var(x) => *env.borrow().lookup(&x),
        Make(box Var(x), box e) => {
            let val = interp_exp(e, Rc::clone(&env));
            env.borrow_mut().bind(x, val);
            0
        },
        Print(box data) => {
            println!("{}", interp_exp(data, Rc::clone(&env)));
            0
        },
        _ => panic!("bad syntax!"),
    }
}

fn main() -> std::io::Result<()> {
    use parser::parse;
    use std::io::{self, Write};

    let mut v = String::new();
    let env = Rc::new(RefCell::new(SymTable::new()));
    loop {
        // input
        print!("User> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut v)?;
        v.pop(); // remove enter
        // parse
        let exp = parse(v.as_str());
        v.clear();
        // interpret
        interp_exp(exp, Rc::clone(&env));
        io::stdout().flush()?;
    }
}