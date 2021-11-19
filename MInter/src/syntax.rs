/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:16:34
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-11-19 14:53:00
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

pub use Expr::*;
pub use ValType::*;

use crate::hashmap;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Mutex;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ListType {
    Ordinary,
    Function(Vec<String>, String),
    Closure,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ValType {
    Int(i64),
    // Float(f64) // I won't implement this I guess.
    Str(String),
    Boolean(bool),
    List(String, ListType),

    ErrorValue,
    // Return value is special dealt with
    Retv(Box<ValType>),
}

impl ValType {
    pub fn is_int(&self) -> bool {
        if let Int(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_string(&self) -> bool {
        if let Str(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_bool(&self) -> bool {
        if let Boolean(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_list(&self) -> bool {
        if let List(_, _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_ret_value(&self) -> bool {
        if let Retv(_) = self {
            true
        } else {
            false
        }
    }
}

impl Into<i64> for ValType {
    fn into(self) -> i64 {
        match self {
            Int(i) => i,
            Str(s) => s.parse().unwrap(),
            Boolean(b) => b as i64,
            List(_, _) => unimplemented!(), // Not supported

            ErrorValue => unimplemented!(),
            Retv(box val) => val.into(), // Typically unreachable
        }
    }
}

impl Into<String> for ValType {
    fn into(self) -> String {
        match self {
            Int(i) => i.to_string(),
            Str(s) => s.clone(),
            Boolean(b) => b.to_string(),
            List(value, _) => value.clone(),

            ErrorValue => "Value Error".into(),
            Retv(box val) => val.into(),
        }
    }
}

impl fmt::Display for ValType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Int(i) => write!(f, "{}", i),
            Str(s) => write!(f, "{}", s),
            Boolean(b) => write!(f, "{}", b),
            List(l, _) => write!(f, "{}", l),

            ErrorValue => write!(f, "{}", "Value Error"),
            Retv(box v) => v.fmt(f),
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

    // For function
    Return(Box<Expr>),
    Function(String, Vec<Expr>),
    Export(Box<Expr>),

    // Others
    Exit,
    ErrorExpr,
}

lazy_static! {
    pub static ref KEYWORD: HashMap<&'static str, i32> = hashmap!(
        "read" => 0, "exit" => 0,
        "print" => 1, "thing" => 1, "erase" => 1, "run" => 1, "export" => 1,
        "isname" => 1, "isnumber" => 1, "isword" => 1, "islist" => 1, "isbool" => 1, "isempty" => 1,
        "not" => 1, "and" => 2, "or" => 2,
        "return" => 1,
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
    T: Eq + Hash + Display + Clone,
    H: Eq + Hash + Display + Clone,
{
    local: HashMap<T, H>,
    global: Option<Rc<RefCell<SymTable<T, H>>>>,
    _pstack: Option<Rc<RefCell<SymTable<T, H>>>>,
}

impl<T, H> SymTable<T, H>
where
    T: Eq + Hash + Display + Clone,
    H: Eq + Hash + Display + Clone,
{
    pub fn new(
        global: Option<Rc<RefCell<SymTable<T, H>>>>,
        pstack: Option<Rc<RefCell<SymTable<T, H>>>>,
    ) -> Self {
        SymTable {
            local: HashMap::new(),
            global: global,
            _pstack: pstack,
        }
    }

    pub fn set_global(&mut self, global: Option<Rc<RefCell<SymTable<T, H>>>>) {
        self.global = global
    }

    pub fn get_global(&self) -> Rc<RefCell<SymTable<T, H>>> {
        Rc::clone(self.global.as_ref().unwrap())
    }

    pub fn exist(&self, x: &T) -> bool {
        self.local.get(x).is_some()
    }

    pub fn exist_global(&self, x: &T) -> bool {
        self.global.as_ref().unwrap().borrow().exist(x)
    }

    pub fn lookup(&self, x: &T) -> H {
        if let Some(h) = self.local.get(x) {
            h.clone()
        } else {
            panic!("Undefine variable {}!", x);
        }
    }

    pub fn lookup_global(&self, x: &T) -> H {
        self.global.as_ref().unwrap().borrow().lookup(x)
    }

    pub fn bind(&mut self, var: T, val: H) -> Option<H> {
        self.local.insert(var, val)
    }

    pub fn export(&mut self, var: T) -> Option<H> {
        let val = self.lookup(&var);
        self.global.as_ref().unwrap().borrow_mut().bind(var, val)
    }

    pub fn unbind(&mut self, var: T) -> Option<H> {
        self.local.remove(&var)
    }
}
