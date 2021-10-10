/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:19:44
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-10-10 19:33:06
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */
use std::io::{self, Write};

pub fn _readint() -> i64 {
    print!("input an integer: ");
    io::stdout().flush().expect("Faild to flush!");
    let mut v = String::new();
    io::stdin().read_line(&mut v).expect("Failed to read line");
    return v.trim().parse().expect("Not an integer!");
}

#[macro_export]
macro_rules! string {
    ( $str:expr ) => {
        String::from($str)
    };
}

#[macro_export]
macro_rules! hashmap {
    ( $( $key:expr => $val:expr ),* ) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert( $key, $val ); )*
            map
        }
    };
}