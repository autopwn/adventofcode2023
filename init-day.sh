#/usr/bin/env bash

DAY_FOLDER="day$(date +%d)"

if [ ! -d "$DAY_FOLDER" ]; then
    echo "Creating $DAY_FOLDER"
    cargo new $DAY_FOLDER
    rm -rf $DAY_FOLDER/.git
    cat template.rs > $DAY_FOLDER/src/main.rs
    echo 'aoc_utils = { path = "/home/andreas/git/aoc/aoc_utils" }' >> $DAY_FOLDER/Cargo.toml
    touch $DAY_FOLDER/example_part1.txt
    touch $DAY_FOLDER/example_part2.txt
fi

