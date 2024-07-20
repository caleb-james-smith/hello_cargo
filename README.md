# hello_cargo

Example cargo project for Rust.

[Rust: Hello Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

Check that cargo is installed:
```
cargo --version
```

Create a new Rust project using cargo:
```
cargo new hello_cargo
cd hello_cargo
```

Check that the project compiles without creating an executable:
```
cargo check
```

Compile and run with two commands (debug version):
```
cargo build
./target/debug/hello_cargo
```

Compile and run with two commands (release version):
```
cargo build --release
./target/release/hello_cargo
```

Compile and run with one command:
```
cargo run
```

