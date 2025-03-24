#!/usr/bin/env bash

root_path=$(dirname $(realpath $0))
root_name=$(basename $root_path)

source $root_path/lib.sh
source $root_path/prepare.sh

dir_src=$root_path/src
dir_out=$root_path/target

function glue_code() {
    local mod_rs="$dir_src/problemset/mod.rs"

    rm -rf $mod_rs && touch $mod_rs

    for p in $dir_src/problemset/*.rs; do
        if [[ 'mod.rs' != `basename $p` ]]; then
            echo "pub mod lcps`basename $p | grep -oP '^\d+(?=\.)'` { pub struct Solution; include!(\"./`basename $p`\"); }" >> $mod_rs
            echo "" >> $mod_rs
        fi
    done

    rustfmt --version 1>/dev/null 2>&1
    if (( 0 == $? )); then
        rustfmt $mod_rs
    fi
}

function main() {
    rustc -g --out-dir $dir_out --edition 2021 --crate-type lib --crate-name $root_name $dir_src/lib.rs \
    && rustc -g -o $dir_out/$root_name --edition 2021 --crate-type bin --crate-name $root_name -L $dir_out --extern $root_name $dir_src/main.rs \
    && $dir_out/$root_name
}

glue_code
main
