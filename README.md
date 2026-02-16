# Recursive shirts

I was thinking what nerdy thing could be screenprinted/embroided on a shirt.
It seemed super funny to me that the shirt could have a functional code that
generated the vector image used to put it on a shirt so I made it.

# Rust

First POC to visualize the idea. Represents [the code 1:1 which is also square](./rust/src/square_uncolor/main.rs).

![](./img/rust/shirt_print.svg)

```sh
cd rust
cargo build --release --bin square_uncolor
cd target/release
# To generate the `shirt_print.svg` one needs to execute the compiled binary
./square_uncolor
```

# Brainfuck

```sh
# Compile the brainfuck interpreter
gcc bf.c -o bf
# Generate the shirt_print.svg
./bf shirt.bf < shirt.bf | tee shirt_print.svg
```
