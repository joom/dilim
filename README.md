# dilim

A structure editor for a simple functional programming language, with Vim-like shortcuts and commands.

Written in Rust, using [the Yew framework](https://yew.rs/), compiled to WebAssembly. I wrote it as my first Rust project, to learn Rust, so the code probably isn't all idiomatic Rust.

[Live here for now.](http://joomy.korkutblech.com/dilim/)

Press `h` to get help, press `Tab` to get suggestions on how to complete commands.

## Installation and Running

If you don't have the Rust WASM bundle tool [trunk](https://github.com/thedodd/trunk), you should install it:
```
cargo install trunk
```

Then you can build the app and run a static file server with this command:
```
trunk serve
```
