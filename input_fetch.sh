#!/bin/bash

year=2022

day=$(date +%-d)
dir=day-$day

## creates a new rust project
cargo new $dir
curl -b $(cat ~/.cookie) "https://adventofcode.com/$year/day/$day/input" \
    > $dir/input

## creates a new main based on boilerplate code - a simple function to load the input data into a string and a main
cp template.rs $dir/src/main.rs