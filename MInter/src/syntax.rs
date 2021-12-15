/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:16:34
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-12-16 00:45:13
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

pub use Expr::*;
pub use ValType::*;

use crate::hashmap;
use lazy_static::lazy_static;
use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Mutex;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ClosureEnv {
    pub name: String,
    pub val: ValType,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ListType {
    Ordinary,
    Function(Vec<ClosureEnv>, Vec<String>, Vec<ValType>),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ValType {
    Num(OrderedFloat<f64>),
    Str(String),
    Boolean(bool),
    List(Vec<ValType>, ListType),

    // When error occurs
    Null,
    // Return value is special dealt with
    Retv(Box<ValType>),
}

impl ValType {
    pub fn is_num(&self) -> bool {
        if let Num(_) = self {
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

    pub fn list_is_func(&self) -> Option<i32> {
        if let List(_, ListType::Function(_, params, _)) = self {
            Some(params.len() as i32)
        } else {
            None
        }
    }

    pub fn is_ret_value(&self) -> bool {
        if let Retv(_) = self {
            true
        } else {
            false
        }
    }

    pub fn get_ret_value(self) -> ValType {
        if let Retv(box v) = self {
            v
        } else {
            ValType::Null
        }
    }

    pub fn find_val_in_list(&self, set: &mut HashSet<String>) {
        if let List(l, _) = self {
            l.iter().map(|v| v.find_val_in_list(set)).count();
        } else if let Str(s) = self {
            let word = if s.starts_with(":") {
                s[1..].to_string()
            } else {
                s.to_string()
            };
            if KEYWORD.get(word.as_str()).is_none() {
                set.insert(word);
            }
        } else {
            panic!("find_val_in_list error, elements invalid")
        }
    }
}

pub fn vec2str(list: &Vec<ValType>) -> String {
    let mut slist = "[ ".to_string();
    for value in list {
        if let List(l, _) = value {
            slist.extend([vec2str(l).as_ref(), " "])
        } else {
            slist.extend([value.to_string().as_ref(), " "])
        }
    }
    slist.push_str("]");
    slist
}

impl Into<OrderedFloat<f64>> for ValType {
    fn into(self) -> OrderedFloat<f64> {
        match self {
            Num(n) => n,
            Str(s) => s.parse().unwrap(),
            Boolean(b) => (b as i64 as f64).into(),
            List(_, _) => unimplemented!(), // Not supported

            Null => unimplemented!(),
            Retv(box val) => val.into(), // Typically unreachable
        }
    }
}

impl fmt::Display for ValType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Int(i) => write!(f, "{}", i),
            Num(n) => write!(f, "{}", n),
            Str(s) => write!(f, "{}", s),
            Boolean(b) => write!(f, "{}", b),
            List(list, _) => write!(f, "{}", vec2str(list)),

            Null => write!(f, "{}", "null"),
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

    // Empty
    Nop,

    // Others
    Exit,
}

lazy_static! {
    pub static ref KEYWORD: HashMap<&'static str, i32> = hashmap!(
        "nop" => 0, "read" => 0, "exit" => 0,
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
pub struct SymTable {
    local: HashMap<String, ValType>,
    global: Option<Rc<RefCell<SymTable>>>,
    context: Option<Rc<RefCell<SymTable>>>,
    func: HashMap<String, i32>,
}

impl SymTable {
    pub fn new(
        global: Option<Rc<RefCell<SymTable>>>,
        context: Option<Rc<RefCell<SymTable>>>,
    ) -> Self {
        SymTable {
            local: HashMap::new(),
            global: global,
            context: context,
            func: HashMap::new(),
        }
    }

    pub fn set_global(&mut self, global: Option<Rc<RefCell<SymTable>>>) {
        self.global = global
    }

    pub fn get_global(&self) -> Rc<RefCell<SymTable>> {
        Rc::clone(self.global.as_ref().unwrap())
    }

    pub fn exist_local(&self, x: &String) -> bool {
        self.local.get(x).is_some()
    }

    pub fn exist_context(&self, x: &String) -> bool {
        self.context.as_ref().unwrap().borrow().exist_local(x)
    }

    pub fn exist_global(&self, x: &String) -> bool {
        self.global.as_ref().unwrap().borrow().exist_local(x)
    }

    pub fn lookup(&self, x: &String) -> ValType {
        if let Some(h) = self.local.get(x) {
            h.clone()
        } else if self.context.is_some() {
            self.lookup_context(x).unwrap()
        } else if self.global.is_some() {
            self.lookup_global(x).unwrap()
        } else {
            panic!("Undefine variable {}!", x);
        }
    }

    pub fn lookup_local(&self, x: &String) -> Option<ValType> {
        self.local.get(x).map(|v| v.to_owned())
    }

    pub fn lookup_context(&self, x: &String) -> Option<ValType> {
        self.context.as_ref().unwrap().borrow().lookup_local(x)
    }

    pub fn lookup_global(&self, x: &String) -> Option<ValType> {
        self.global.as_ref().unwrap().borrow().lookup_local(x)
    }

    pub fn bind(&mut self, var: String, val: ValType) -> Option<ValType> {
        if let Some(param_num) = val.list_is_func() {
            self.add_func(&var, param_num);
        }
        self.local.insert(var, val)
    }

    pub fn export(&mut self, var: String) -> Option<ValType> {
        let val = self.lookup_local(&var).unwrap();
        self.global.as_ref().unwrap().borrow_mut().bind(var, val)
    }

    pub fn unbind(&mut self, var: String) -> Option<ValType> {
        self.remove_func(&var);
        self.local.remove(&var)
    }

    fn add_func(&mut self, func_name: &str, param_num: i32) -> Option<i32> {
        self.func.insert(func_name.to_string(), param_num)
    }

    fn remove_func(&mut self, func_name: &str) -> Option<i32> {
        self.func.remove(func_name)
    }

    pub fn is_func(&self, func_name: &str) -> Option<i32> {
        self.func.get(func_name).map(|&i| i)
    }
}
