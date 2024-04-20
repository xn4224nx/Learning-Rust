#!/bin/bash

filenm="archive.map"
OLDIFS=$IFS; 
IFS=',';

cargo build -q

echo -e "\nAdd Some Key Value Pairs\n"
for i in cream,three eight,road red,name seven,ninety none,forty eight,seven; 
    do 
    set -- $i; 
    cargo run -q $filenm insert $1 $2
done; 

echo -e "\nExtract Some Values\n"
cargo run -q $filenm get seven
cargo run -q $filenm get none
cargo run -q $filenm get cream
cargo run -q $filenm get no_key_exists

echo -e "\nDelete Some Values\n"
cargo run -q $filenm delete seven
cargo run -q $filenm delete cream

echo -e "\nUpdate Some Others\n"
cargo run -q $filenm update red four
cargo run -q $filenm update none test

echo -e "\nSee the Results\n"
cargo run -q $filenm get seven
cargo run -q $filenm get none
cargo run -q $filenm get cream
cargo run -q $filenm get red

# Cleanup
rm $filenm
cargo -q clean

