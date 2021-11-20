#!/bin/bash
###
 # @Author: Yinwhe
 # @Date: 2021-10-24 12:53:25
 # @LastEditors: Yinwhe
 # @LastEditTime: 2021-11-20 12:50:03
 # @Description: file information
 # @Copyright: Copyright (c) 2021
### 
#echo "cleaning"
#echo "compiling"
javac -cp .:mua/src O.java
# rustup default nightly
cd MInter
cargo build