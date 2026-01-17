# semire-core-utils 🦀✨

A small Rust utility crate that makes **reading input** and **basic file operations** easier and less repetitive.

This crate provides:

* A generic way to read typed input from `stdin`
* Simple helpers for creating, writing to, and reading files
* Clean error handling using `Result`

Perfect for small projects, CLIs, learning Rust, or when you’re tired of rewriting the same boilerplate.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
serenity-utils = "0.1.0"
```

---

## Usage Examples

### 1️⃣ Reading typed input from stdin

The `Readable` trait allows you to read user input into **any type that implements `FromStr`**.

```rust
use serenity_utils::Readable;

let age: u32 = Readable::read();
let username: String = Readable::read();
```

Behavior:

* Keeps retrying until valid input is entered
* Handles empty input
* Prints helpful error messages automatically

---

### 2️⃣ Creating a file

Create a file with optional overwrite behavior:

```rust
use serenity_utils::create_file;

// Overwrite if the file already exists
create_file("data.txt", true)?;

// Keep existing contents if the file exists
create_file("data.txt", false)?;
```

---

### 3️⃣ Writing data to a file

Write text data to a file with append or overwrite control:

```rust
use serenity_utils::write_data;

write_data(
    "clients.serenity",
    "New client added",
    true,  // append to file
    false, // do not overwrite
)?;
```

Notes:

* Automatically appends a newline
* Prints a success message on completion

---

### 4️⃣ Reading a file

Read and print the contents of a file:

```rust
use serenity_utils::read_data;

read_data("clients.serenity")?;
```

Details:

* Reads the entire file as raw bytes
* Converts safely to UTF-8 using `from_utf8_lossy`
* Prints contents to stdout

---

## Full Example

```rust
use serenity_utils::{Readable, create_file, write_data, read_data};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter your name:");
    let name: String = Readable::read();

    create_file("users.txt", true)?;
    write_data("users.txt", &name, true, false)?;
    read_data("users.txt")?;

    Ok(())
}
```

---

## Error Handling

Most functions return `Result`, allowing easy propagation with `?`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_file("example.txt", true)?;
    Ok(())
}
```

---

## Design Notes

* Uses `OpenOptions` for flexible file handling
* Uses `Box<dyn Error>` where flexibility is preferred
* Designed to be small, explicit, and beginner-friendly

---

## License

MIT License

---

If this crate saves you from rewriting the same code twice, it’s doing its job 😌
