#!/usr/bin/env rust-script

// rustc --explain E0308
//cargo run --bin abs -- -5 -3 -4 2 43 1 3 0

fn abs(x: i64) -> u64 {
    (if x < 0 { -x } else { x }) as u64
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);

    loop {
        let number = args.next().ok_or("Not enough arguments")?.parse()?;
        println!("abs({number}) = {}", abs(number));
        match number.cmp(&0) {
            std::cmp::Ordering::Less => {
                eprintln!("Note: {number}");
                continue;
            }
            std::cmp::Ordering::Greater => {
                continue;
            }
            std::cmp::Ordering::Equal => {
                break;
            }
        };
    }

    Ok(())
}
