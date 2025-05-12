# 🔗 linker

A terminal-based personal link manager written in Rust. Save links with tags, search them easily, and open them directly from the command line.

---

## 🚀 Installation

### Using Cargo (recommended)

```bash
cargo install linker
```

> Make sure `~/.cargo/bin` is in your `$PATH`.

### Manual Build

```bash
git clone https://github.com/ChristianRegueiro/linker.git
cd linker
cargo build --release
./target/release/linker
```

---

## 🧰 Available Commands

```bash
linker add <TITLE> <URL> --tags tag1,tag2
linker list
linker search <TEXT>
linker open <ID>
linker remove <ID>
```

---

## 📦 Examples

### Add a link

```bash
linker add "Rust Docs" https://doc.rust-lang.org --tags rust,docs
```

### List all saved links

```bash
linker list
```

### Search by text (title, URL, or tags)

```bash
linker search rust
```

### Open a link in the browser

```bash
linker open 1
```

### Remove a link

```bash
linker remove 1
```

---

## 📂 Storage

Links are stored locally in:

```
~/.linker/links.json
```

Simple JSON format. No external database required.

---

## 🛠 Key Dependencies

- [`clap`](https://crates.io/crates/clap): CLI argument parser
- [`serde`](https://crates.io/crates/serde): JSON serialization
- [`colored`](https://crates.io/crates/colored): color output
- [`open`](https://crates.io/crates/open): open URLs in the browser
- [`chrono`](https://crates.io/crates/chrono): date and time handling
- [`dirs`](https://crates.io/crates/dirs): cross-platform user directories

---

## 📜 License

MIT © [Christian Regueiro](https://github.com/ChristianRegueiro)
