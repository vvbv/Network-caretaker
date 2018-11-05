#!/bin/bash
clear
cd src
echo "Compiling"
cargo build
echo "Compiling [Completed]"
echo "Running\n"
cargo run
echo "\nDone, return code $?"