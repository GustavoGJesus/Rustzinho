# Rust Learning 🦀
Guide: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
1. Create project with cargo!

```rust
$ cargo new hello_cargo
$ cd hello_cargo
```

2. Connect your repository 

```rust
$ cargo new <path your directory>
```

3. Building and executing a project with cargo:

```rust
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs

$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

4. Importantly, the best option is to use the command:

```rust
 $ cargo check
```

This command checks your code, but it doesn't generate an executable, it just checks if your code is compiling

5. Build for release, this command is used to build your project for delivery to the user, recommend using this command only when your project is ready.

```rust
$ cargo build --release
```