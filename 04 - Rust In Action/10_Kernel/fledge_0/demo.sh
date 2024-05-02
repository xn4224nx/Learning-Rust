#!/bin/bash

# Ensure QEMU is installed
2>/dev/null 1>&2 qemu-system-x86_64 --version
ret_code=$?

if ! [ $ret_code -eq 0 ]; then 
    echo "QEMU Not installed" 1>&2
    sudo apt install qemu-system
fi
