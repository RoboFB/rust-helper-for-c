# install

### Download
option 1. use the binary [x86_64-unknown-linux-musl](https://github.com/RoboFB/rust-helper-for-c/releases/tag/release)  
option 2. built with cargo from source for example `cargo build --release --target x86_64-unknown-linux-musl`

### Move
at the binary to your env path or make an alias to it

# Usage
go in a C project it needs:
* `src/` with `*.c` fils
* `Makefile` with `SRC :=`
* `include/function_definitions.h` with `// start` and `// end`


