#!/usr/bin/env rust-script

fn main() {
    for _ in 0..2 {
        println!("1");
    }

    let mut i: u64 = 1;
    let mut j = i;

    while j < 1000 {
        let k = i + j;
        i = j;
        j = k;

        println!("{k}");
    }
}
