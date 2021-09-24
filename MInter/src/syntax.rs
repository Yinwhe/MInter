/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:16:34
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-09-24 11:16:34
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

pub enum Expr {
    Int(i64),
    Prim0(String),
    Prim1(String, Box<Expr>),
    Prim2(String, Box<Expr>, Box<Expr>),
}

pub use Expr::*;