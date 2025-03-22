#!/usr/bin/env bash

root_path=$(dirname $(realpath $0))
root_name=$(basename $root_path)

source $root_path/lib.sh
source $root_path/prepare.sh

dir_src=$root_path/src
dir_out=$root_path/target

rustc_cfg='--cfg lc9 --cfg lc66'

function main() {
    rustc -g --out-dir $dir_out --edition 2021 --crate-type lib --crate-name $root_name $rustc_cfg $dir_src/lib.rs \
    && rustc -g -o $dir_out/$root_name --edition 2021 --crate-type bin --crate-name $root_name -L $dir_out --extern $root_name $dir_src/main.rs \
    && $dir_out/$root_name
}
main
