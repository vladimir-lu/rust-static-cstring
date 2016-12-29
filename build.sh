#!/bin/sh -eu
cargo build
gcc -L target/debug -l cstring -o test test.c
LD_LIBRARY_PATH=`pwd`/target/debug ./test
