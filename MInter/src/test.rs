/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:21:13
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-09-24 11:45:58
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

use crate::*;
use parser::*;

#[test]
fn test_r0() {
    let p3 = Prim2 ("+".to_string(), Box::new(Int(10)), Box::new(Int(32)));
    let r = interp_exp(p3);
    assert_eq!(r, 42);
}

#[test]
fn test_scan() {
    let s = "(1 2 (+ 1 2))";
    let expr = scan(s);
    let t = List(vec![Atom("1".to_string()), Atom("2".to_string()), 
                        List(vec![Atom("+".to_string()), Atom("1".to_string()), Atom("2".to_string())])]);
    assert_eq!(expr, t);
}

#[test]
fn test_interp() {
    let s = "(+ 1 2)";
    let expr = parse(s);
    let r = interp_exp(expr);
    assert_eq!(r, 3);
}