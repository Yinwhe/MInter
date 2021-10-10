/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:16:34
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-10-10 20:16:16
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ValType{
    Int(i64),
    // Float(f64),
    Str(String),
    // List(),
    // Boolean()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Value(ValType),
    Var(String),
    Make(Box<Expr>, Box<Expr>),
    Print(Box<Expr>),
    Thing(Box<Expr>)
}
pub use Expr::*;

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct SymTable<T, H>
where
    T: Eq + Hash,
    H: Eq + Hash,
{
    pub map: HashMap<T, H>,
    // env: Option<Rc<SymTable<T, H>>>,
}

impl<T, H> SymTable<T, H>
where
    T: Eq + Hash,
    H: Eq + Hash,
{
    pub fn new() -> Self {
        SymTable {
            map: HashMap::new(),
        }
    }
    pub fn lookup(&self, x: &T) -> &H {
        if let Some(h) = self.map.get(x) {
            return h;
        } else {
            panic!("Undefine variable!");
        }
    }

    pub fn bind(&mut self, var: T, val: H) -> Option<H> {
        return self.map.insert(var, val);
    }
}
