#!/bin/bash
#echo "cleaning"
#echo "compiling"
javac -cp .:mua/src O.java
rustup default nightly
cd MInter
cargo build