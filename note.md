# Rust Package Build Command

## Library Crate

- rustc --crate-type lib --crate-name [crate-name] src/lib.rs -o lib[crate-name].rlib

## Binary Crate

- rustc src/main.rs --extern [crate-name]=lib[crate-name].rlib -o minigrep
