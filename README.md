# 🔗 linker

A terminal-based personal link manager written in Rust. Save links with tags, search them easily, and open them directly from the command line.

[![Crates.io](https://img.shields.io/crates/d/linker-cli)](https://crates.io/crates/linker-cli)

<a href="https://ko-fi.com/christianregueiro/tip" target="_blank">
  <img src="docs/ko-fi-button.png" alt="Buy Me a Coffee" height="35"/>
</a>

---

## 🚀 Installation

### Using yay

```bash
yay -S linker-cli
```

### Using Cargo (recommended)

```bash
cargo install linker-cli
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
linker-cli add <TITLE> <URL> --tags tag1,tag2
linker-cli list
linker-cli search <TEXT>
linker-cli open <ID or TITLE>
linker-cli edit <ID or TITLE>
linker-cli remove <ID>
```

---

## 📦 Examples

### Add a link

```bash
linker-cli add "Rust Docs" https://doc.rust-lang.org --tags rust,docs
```

### List all saved links

```bash
linker-cli list
```

### Search by text (title, URL, or tags)

```bash
linker-cli search rust
```

### Open a link in the browser

```bash
linker-cli open 1
linker-cli open rust
```

### Remove a link

```bash
linker-cli remove 1
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
