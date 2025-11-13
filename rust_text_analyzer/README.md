# 🧩 Text Analyzer CLI (Rust)

A simple but solid **command-line text analyzer** written in **Rust**.

Give it a text file and it will compute:

- Number of **lines**
- Number of **characters** (with and without spaces)
- Number of **words**
- Number of **unique words**
- **Average word length**
- **Top N most frequent words**

This project shows off **Rust**, **CLI tools**, **file I/O**, **ownership**, **hash maps**, and **sorting**.

---

## 🛠 Tech & Techniques

- Language: **Rust**
- Uses:
  - `std::fs` for file reading
  - `std::collections::HashMap` for word frequencies
  - Command-line arguments via `std::env::args`
  - Simple early-exit error handling via `Result` and `process::exit`
  - A data struct (`TextStats`) to organize metrics

---

## ▶️ How to Build & Run

### 1. Install Rust (if needed)

If you don't have Rust yet:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the instructions, then restart your terminal.

---

### 2. Build the project

Inside the project folder:

```bash
cargo build
```

This compiles the `text_analyzer` binary.

---

### 3. Run on a text file

Create a test file, for example:

```bash
echo "Hello world. Hello GitHub portfolio!" > sample.txt
```

Then run:

```bash
cargo run -- sample.txt
```

Or specify a custom "top N" count for frequent words:

```bash
cargo run -- sample.txt 15
```

You’ll see stats like:

```text
=============================
  Text Analyzer (Rust CLI)
=============================

Lines:                 1
Characters (with spaces):    38
Characters (no spaces):      32
Words:                        5
Unique words:                 4
Average word length:          5.20

Top 10 words:
-----------------------------
 1. hello           2
 2. github          1
 3. portfolio       1
 4. world           1
```

---

## 📂 Project Structure

```text
text_analyzer/
├─ Cargo.toml
└─ src/
   └─ main.rs
```

---

## 🌟 Why this looks good on GitHub

This project demonstrates that you can:

- Use **Rust** (a modern systems language)
- Build **CLI tools** with arguments
- Handle **file I/O and errors**
- Use **collections** (hash maps, vectors) and sorting
- Structure code cleanly with functions and structs

You can extend it later with:

- Stopword filtering
- Exporting results to JSON/CSV
- Colored terminal output
