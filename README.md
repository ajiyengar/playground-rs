# playground-rs

## Initialize your playground

```sh
cargo new playground-rs
cd playground-rs
mkdir -p src/bin
mv src/main.rs src/bin/hello.rs
cargo run --bin hello
```

## Add more programs

```sh
echo 'fn main() {}' > src/bin/nextapp.rs
# Build (this makes the analyzer happy)
cargo build
vim src/bin/nextapp.rs
cargo run --bin nextapp
```

## Make Rust scripts directly executable

```sh
cargo install rust-script
sed -i '1i#!/usr/bin/env rust-script' src/bin/hello.rs
chmod a+rx src/bin/hello.rs
./src/bin/hello.rs
```

⇒ Hello, world!
