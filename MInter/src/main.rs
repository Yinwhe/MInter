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

mod helper;
mod parser;
mod syntax;
mod interp;
mod cmdin;

pub use crate::syntax::Expr::{self, *};
pub use crate::syntax::SymTable;
pub use crate::cmdin::Input;

use std::cell::RefCell;
use std::io::BufRead;
use std::rc::Rc;

fn main() {
    use parser::parse;
    use interp::interp_exp;
    
    let env = Rc::new(RefCell::new(SymTable::new()));

    let mut input = Input::file("../in").unwrap().lines();

    while let Some(Ok(v)) = input.next() {
        let exp = parse(v.as_str());
        interp_exp(&mut input, exp, Rc::clone(&env));
    }
}