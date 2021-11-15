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

mod cmdin;
mod helper;
mod interp;
mod parser;
mod syntax;

pub use crate::cmdin::Input;
pub use crate::syntax::Expr::{self, *};
pub use crate::syntax::SymTable;

use std::cell::RefCell;
use std::io::{BufRead, Write};
use std::process::exit;
use std::rc::Rc;

fn main() {
    use crate::interp::interp_exp;
    use crate::parser::parse;

    let env = Rc::new(RefCell::new(SymTable::new()));

    let file = std::env::args().nth(1);
    match file {
        Some(filename) => {
            let mut input = Input::file(&filename).unwrap().lines();
            loop {
                let exps = parse(&mut input);
                if exps.is_empty() {
                    exit(0);
                }
                exps.into_iter()
                    .map(|exp| interp_exp(&mut input, exp, Rc::clone(&env)))
                    .last();
            }
        }
        None => {
            let stdin = std::io::stdin();
            let mut input = Input::console(&stdin).lines();
            loop {
                print!("User>");
                std::io::stdout()
                    .flush()
                    .expect("Fatal error! Stdout flush fails!");
                parse(&mut input)
                    .into_iter()
                    .map(|exp| interp_exp(&mut input, exp, Rc::clone(&env)))
                    .last();
            }
        }
    }
}
