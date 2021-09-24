/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:19:44
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-09-24 11:20:53
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */
use std::io::{self, Write};

pub fn readint() -> i64 {
    print!("input an integer: ");
    io::stdout().flush().expect("Faild to flush!");
    let mut v = String::new();
    io::stdin().read_line(&mut v).expect("Failed to read line");
    return v.trim().parse().expect("Not an integer!");
}