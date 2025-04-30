#!/bin/bash
set -xeu

RUSTFLAGS="--C instrument-coverage" cargo test
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/ --excl-start "#\[cfg\(test\)\]" --keep-only **/src/**/*RUSTFLAGS="--C instrument-coverage" 
