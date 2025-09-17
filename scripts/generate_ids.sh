#!/bin/bash

for i in {1..8}; do
    # Generate 16 random bytes (128 bits) and format as hex with 0x prefix
    od -An -tx1 -N16 /dev/urandom | tr -d ' \n' | sed 's/^/0x/' | tr 'a-f' 'A-F'
    echo
done