# playground-rs

## Initialize playground

```sh
cargo new playground-rs
cd playground-rs
mkdir -p src/bin
mv src/main.rs src/bin/app.rs
cargo build   ## To make rust-analyzer happy in vim
vim src/bin/app.rs
cargo run --bin app
```

## Make Rust scripts directly executable

```sh
cargo install rust-script
sed -i '1i#!/usr/bin/env rust-script' src/bin/app.rs
chmod a+rx src/bin/app.rs
./src/bin/app.rs
```

⇒ Hello, world!

## REPL for experimentation

```sh
cargo install evcxr_repl
evcxr
```

```rs
>> fn whatis<T>(v: T) -> T {
  println!("{}", std::any::type_name::<T>());
  v
}
>> whatis((0..10).skip(3).take(2))
core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::ops::range::Range<i32>>>
Take { iter: Skip { iter: 0..10, n: 3 }, n: 2 }
```

## lldb Debugger

Read 64-bit value:  

```c
    expr/x -- *((uint64_t *)0x16fdfc370)

```

Write 64-bit value:  

```c
    expr/x -- *((uint64_t *)0x16fdfc370) = 0x3ff4cccccccccccd
```
