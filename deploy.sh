#!/bin/bash
clear
echo "Clearing old binaries"
rm build/network_observant > /dev/null
echo "Clearing old binaries [Completed]"
echo "Compiling"
rustc src/main.rs -o build/network_observant
echo "Compiling [Completed]"
echo "Running\n"
build/network_observant
echo "\nDone, return code $?"