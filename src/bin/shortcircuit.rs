#!/usr/bin/env rust-script

fn bravo() -> bool {
    println!("bravo is true");
    true
}

fn alpha() -> bool {
    println!("alpha is false");
    false
}

fn main() {
    println!("expr is {}", alpha() & bravo());
    println!("expr is {}", bravo() | alpha());
    println!("expr is {}", alpha() && bravo());
    println!("expr is {}", bravo() || alpha());
}
