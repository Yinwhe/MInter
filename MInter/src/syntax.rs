/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:16:34
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-11-17 08:11:19
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

pub use Expr::*;
pub use ValType::*;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;
use crate::hashmap;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ListType {
    Ordinary,
    Function(Vec<String>, String),
    Closure,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ValType{
    Int(i64),
    // Float(f64) // I won't implement this I guess.
    Str(String),
    Boolean(bool),
    List(String, ListType)
}

impl Into<i64> for ValType {
    fn into(self) -> i64 {
        match self {
            Int(i) => i,
            Str(s) => s.parse().unwrap(),
            Boolean(b) => b as i64,
            List(_, _) => unimplemented!() // Not supported
        }
    }
}

impl Into<String> for ValType {
    fn into(self) -> String {
        match self {
            Int(i) => i.to_string(),
            Str(s) => s.clone(),
            Boolean(b) => b.to_string(),
            List(value, _) => value.clone()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    // Variables
    Value(ValType),
    Var(String),

    // Operation
    Read(),
    Erase(Box<Expr>),
    Print(Box<Expr>),
    Thing(Box<Expr>),
    Run(Box<Expr>),
    Judge(String, Box<Expr>),
    Make(Box<Expr>, Box<Expr>),
    Comp(String, Box<Expr>, Box<Expr>),
    Calc(String, Box<Expr>, Box<Expr>),
    Logic(String, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    
    Function(String, Vec<Expr>)
}
lazy_static!{
    pub static ref KEYWORD: HashMap<&'static str, i32> = hashmap!(
        "read" => 0,
        "print" => 1, "thing" => 1, "erase" => 1, "run" => 1,
        "isname" => 1, "isnumber" => 1, "isword" => 1, "islist" => 1, "isbool" => 1, "isempty" => 1,
        "not" => 1,
        "and" => 2, "or" => 2,
        "eq" => 2, "gt" => 2, "lt" => 2,
        "add" => 2, "sub" => 2, "mul" => 2, "div" => 2, "mod" => 2,
        "make" => 2,
        "if" => 3
    );

    pub static ref FUNC_NAME: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
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

    pub fn exist(&self, x: &T) -> bool {
        self.map.get(x).is_some()
    }

    pub fn lookup(&self, x: &T) -> &H {
        if let Some(h) = self.map.get(x) {
            h
        } else {
            panic!("Undefine variable!");
        }
    }

    pub fn bind(&mut self, var: T, val: H) -> Option<H> {
        self.map.insert(var, val)
    }

    pub fn unbind(&mut self, var: T) -> Option<H> {
        self.map.remove(&var)
    }
}
