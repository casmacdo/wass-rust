# Rust and WebAssembly

Rust is a low-level programming language that Mozilla developed to overcome shortcommings of languages like C and C++.

WebAssembly acts as a compilation target and allows low-level programming languages like C, C++, and Rust to run on the web.

With WebAssembly, faster, more efficent programming languages can run on the web.

[Mozilla provides more information about WebAssembly on the page linked to this text.](https://developer.mozilla.org/en-US/docs/WebAssembly)

This repository is based on [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)

If you're interested in learning how to use Rust and WebAssembly, you're probably better off accessing the resources linked above. Below, I explain how to reproduce this repository and how the code works as a means to further my knowledge.

## Getting started

First, we need to install Rust. Please follow instructions on [this page](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm) should you need to install rust.

After we install Rust, we need to install wasm-pack, which compiles the code to WebAssembly and produces files that will work in user's browsers. To install wasm-pack, we run the following code:

```bash
cargo install wasm-pack
```

Now that we have wasm-pack installed, we can create our rust package.

Rust makes this process really easy. All we need to do is open a command line instance in the directory where we would like to store our code and use Rust's cargo command.

```bash
cargo new --lib wass-rust
```
more to come...