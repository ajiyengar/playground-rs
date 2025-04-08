#!/usr/bin/env rust-script

fn main() {
    let mut i = 0;
    let j = 0;

    let mi = &mut i;
    *mi = 1;

    let ii = &i; // OK: if mutable reference mi remains unused
    println!("{ii}");
    // *ii = 2; // ERR: not a mutable reference

    // *mi = 3; // ERR: above 'let ii' is an immutable borrow
    // Each item can only ever be accessed either via a unique mutable refrence
    // (read/write) or a shared immutable reference (read-only)

    let ij = &j;
    // *ij = 1; // ERR: not a mutable reference

    // let mj = &mut j; // ERR: cannot borrow as mutable
}
