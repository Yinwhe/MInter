/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:12:25
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-10-12 20:17:44
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */
#![feature(box_patterns)]
extern crate lazy_static;
extern crate simple_logger;
extern crate log;

mod helper;
mod parser;
mod syntax;
mod interp;

pub use crate::syntax::Expr::{self, *};
pub use crate::syntax::SymTable;

use std::cell::RefCell;
use std::rc::Rc;

fn main() -> std::io::Result<()> {
    use parser::parse;
    use interp::interp_exp;
    use std::io::{self, Write};
    use simple_logger::SimpleLogger;

    SimpleLogger::new().env().init().unwrap();
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