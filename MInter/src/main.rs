/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:12:25
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-12-15 23:27:28
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */
#![feature(box_patterns)]
extern crate ansi_term;
extern crate lazy_static;
extern crate num_traits;
extern crate ordered_float;
extern crate regex;

mod cmdin;
mod helper;
mod interp;
mod parser;
mod syntax;

pub use crate::cmdin::Input;
pub use crate::syntax::Expr::{self, *};
pub use crate::syntax::SymTable;

use ansi_term::Color;
use std::cell::RefCell;
use std::io::Write;
use std::process::exit;
use std::rc::Rc;

fn main() {
    use crate::interp::interp_exp;
    use crate::parser::parse;

    let global = Rc::new(RefCell::new(SymTable::new(None, None)));
    global.borrow_mut().set_global(Some(Rc::clone(&global)));

    let file = std::env::args().nth(1);
    match file {
        Some(filename) => {
            let mut input = Input::file(&filename);
            while let Some(expr) = parse(&mut input, Rc::clone(&global)) {
                interp_exp(&mut input, expr, Rc::clone(&global));
            }
            exit(0)
        }
        None => {
            let stdin = std::io::stdin();
            let mut input = Input::console(&stdin);
            loop {
                print!("{}", Color::Green.paint("User>"));
                std::io::stdout()
                    .flush()
                    .expect("Fatal error! Stdout flush fails!");
                let exp = parse(&mut input, Rc::clone(&global)).unwrap();
                interp_exp(&mut input, exp, Rc::clone(&global));
            }
        }
    }
}
