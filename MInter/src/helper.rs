/*
 * @Author: Yinwhe
 * @Date: 2021-09-24 11:19:44
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-12-21 20:54:23
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */

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

#[macro_export]
macro_rules! vecdeque {
    ($($x:expr),*) => (
        VecDeque::from([$($x), *])
    )
}