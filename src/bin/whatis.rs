#!/usr/bin/env rust-script

/// Returns a (statically allocated) string literal describing the type of the argument.
fn whatis<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn ignore(_x: u64) {}

fn main() {
    println!("{}", whatis(1)); //i32
    println!("{}", whatis(1 + 41)); //i32
    let x = 1; // type can be deduced from ignore() arguments

    ignore(x);
    println!("{}", whatis(x)); //u64
    println!("{}", whatis(x + 41)); //u64

    // y's type
    let y = 1;
    let _z = x + y; // type of y (and z) can be deduced from type of x
    println!("{}", whatis(y)); //u64
    println!("{}", whatis(y as i16 + 41)); //i16
}
