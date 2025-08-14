# Installation

You can install **statictranspl** in 3 different ways. You can either download it from **eiipm**: ewwii's own package manager, build it from source, or install it with cargo.

It is suggested to install with **eiipm** as it is the package manager of ewwii, but other options work perfectly fine as well.

## Installing via eiipm

```bash
# install statictranspl
eiipm install statictranspl

# run statictranspl
statictranspl --help
```

## Installing via cargo

```bash
# install statictranspl
cargo install https://github.com/Ewwii-sh/statictranspl/

# run statictranspl
statictranspl --help
```

## Compiling from source

```bash
# clone the repo with git
git clone https://github.com/Ewwii-sh/statictranspl.git

# go into the statictranspl directory
cd statictranspl

# build from source
cargo build --release

# run statictranspl
./target/release/statictranspl --help
```
