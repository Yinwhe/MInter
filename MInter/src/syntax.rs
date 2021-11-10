/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:16:34
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-10-12 19:33:30
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

pub use Expr::*;
pub use ValType::*;
use lazy_static::lazy_static;
use core::panic;
use std::collections::HashMap;
use std::hash::Hash;
use crate::hashmap;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ValType{
    Int(i64),
    // Float(f64) // I won't implement this I guess.
    Str(String),
    List(Vec<ValType>), // Honestly, I treat list as a string. (I'm lazy!)
    Boolean(bool)
}

impl Into<i64> for ValType {
    fn into(self) -> i64 {
        match self {
            Int(i) => i,
            Str(s) => s.parse().unwrap(),
            Boolean(b) => b as i64,
            List(_) => unreachable!() // Not supported
        }
    }
}

impl Into<String> for ValType {
    fn into(self) -> String {
        match self {
            Int(i) => i.to_string(),
            Str(s) => s.clone(),
            Boolean(b) => b.to_string(),
            List(_) => unreachable!() // Not supported
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Value(ValType),
    Var(String),
    Make(Box<Expr>, Box<Expr>),
    Erase(Box<Expr>),
    Print(Box<Expr>),
    Thing(Box<Expr>),
    Run(Box<Expr>),
    Calc(String, Box<Expr>, Box<Expr>),
    Read()
}

lazy_static!{
    pub static ref VALID_OP: HashMap<&'static str, i32> = hashmap!(
        "read" => 0,
        "print" => 1, "thing" => 1, "erase" => 1, "run" => 1,
        "make" => 2, "add" => 2, "sub" => 2, "mul" => 2, "div" => 2, "mod" => 2);
}

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

    pub fn unbind(&mut self, var: T) -> Option<H> {
        return self.map.remove(&var);
    }
}
