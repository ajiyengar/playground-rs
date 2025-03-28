#!/usr/bin/env rust-script

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let number: u64 = args.next().ok_or("Not enough arguments")?.parse()?;
    for arg in args {
        println!("Ignoring additional argument: {}", arg);
    }
    Ok(std::thread::sleep(std::time::Duration::from_secs(number)))
}