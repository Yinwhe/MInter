/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:12:25
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-10-11 15:05:49
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */
#![feature(box_patterns)]
extern crate lazy_static;

mod helper;
mod parser;
mod syntax;
mod interp;

pub use crate::syntax::Expr::{self, *};
pub use crate::syntax::SymTable;
use std::cell::RefCell;
use std::rc::Rc;
use interp::interp_exp;

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