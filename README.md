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
# To print the end state of N memory tape cells use
./bf shirt.bf -d N < shirt.bf \
    > >(tee shirt_print.svg) \
    2> >(cat >&2)
```

# Acknowledgement

- [_Print value of cell x as number for ANY sized cell_ from esolangs.org](https://esolangs.org/wiki/Brainfuck_algorithms#Print_value_of_cell_x_as_number_for_ANY_sized_cell_\(eg_8bit,_100000bit_etc)
- [string_to_brainfuck.py](./scripts/string_to_brainfuck.py) strongly based and inspired on [GorkyR/brainfuck-text-generator](https://github.com/GorkyR/brainfuck-text-generator/tree/master)
