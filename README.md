# c8asm
c8asm is a Chip-8 assembler.

The syntax for this assembler comes  from Cowgod's chip-8 technical reference which
can be found [here](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

## Building
c8asm is written in rust and can be built using cargo:

```sh
git clone https://github.com/francis36012/c8asm
cd c8asm
cargo build --release
```

## Install
You can install using cargo:

```sh
cargo install --git https://github.com/francis36012/c8asm
```

## Running
```sh
c8asm -i <input-file> -o <output-file>
```
