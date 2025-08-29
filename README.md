# Learn Rust Programming Language

Welcome to **Learn Rust Programming Language**! This repository is designed to help you get started with Rust, one of the fastest-growing systems programming languages focused on safety, concurrency, and performance.

---

## 🚀 What is Rust?

Rust is a modern, statically typed, compiled language that prevents many common programming errors through its unique ownership model and type system. With powerful features and a vibrant community, Rust is great for building everything from embedded devices to web servers and CLI tools.

---

## 🏁 Getting Started

### 1. Install Rust

The recommended way to install Rust is via [rustup](https://rustup.rs/):
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
After installation, verify:
```sh
rustc --version
cargo --version
```

### 2. Your First Program

Create a file called `main.rs`:
```rust
fn main() {
    println!("Hello, world!");
}
```
Compile and run:
```sh
rustc main.rs
./main
```
Or use Cargo:
```sh
cargo new hello_rust
cd hello_rust
cargo run
```

---

## 📚 Topics Covered

- **Basic concepts**: variables, mutability, data types, functions, control flow
- **Ownership, borrowing, lifetimes**
- **Structs and enums**
- **Error handling**
- **Practical exercises and examples**

---

## 📝 Example Code

> See the `examples/` directory for sample code for each topic.

```rust
// Variables and mutability
let mut counter = 0;
counter += 1;

// Structs
struct User {
    name: String,
    age: u8,
}
```

---

## 🎯 Recommended Learning Path

1. [The Rust Book](https://doc.rust-lang.org/book/)
2. [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
3. [Rustlings Exercises](https://github.com/rust-lang/rustlings)

---

## 💡 Contributing

Contributions are welcome! Please open an issue or pull request with ideas, improvements, or additional examples.

---

## 📄 License

This repository is licensed under the MIT License.

---

**Happy learning and coding in Rust! 🦀**
