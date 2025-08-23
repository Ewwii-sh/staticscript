# Installation

You can install **StaticScript** in 3 different ways. You can either download it from **eiipm**: ewwii's own package manager, build it from source, or install it with cargo.

It is suggested to install with **eiipm** as it is the package manager of ewwii, but other options work perfectly fine as well.

## Installing via eiipm

```bash
# install staticscript
eiipm install staticscript

# run staticscript compiler
stsc --help
```

## Installing via cargo

```bash
# install staticscript
cargo install --git https://github.com/Ewwii-sh/staticscript/

# run staticscript cpmpiler
stsc --help
```

## Compiling from source

```bash
# clone the repo with git
git clone https://github.com/Ewwii-sh/staticscript.git

# go into the staticscript directory
cd staticscript

# build from source
cargo build --release

# run staticscript compiler
./target/release/stsc --help
```
