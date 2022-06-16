# HomeDisk cloud server

[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
[total-lines]: https://img.shields.io/tokei/lines/github/MedzikUser/HomeDisk?style=for-the-badge&logo=github&color=fede00
[code-size]: https://img.shields.io/github/languages/code-size/MedzikUser/HomeDisk?style=for-the-badge&color=c8df52&logo=github
[ci]: https://img.shields.io/github/workflow/status/MedzikUser/HomeDisk/Rust/main?style=for-the-badge

[home-screenshot]: https://cdn.medzik.xyz/fz4QGfS.png
[login-screenshot]: https://cdn.medzik.xyz/vo10bes.png

[![docs-rs]](https://homedisk-doc.vercel.app)
[![total-lines]](https://github.com/MedzikUser/HomeDisk)
[![code-size]](https://github.com/MedzikUser/HomeDisk)
[![ci]](https://github.com/MedzikUser/HomeDisk/actions/workflows/rust.yml)

![home-screenshot]
![login-screenshot]

## 👨‍💻 Building

First clone the repository: `git clone https://github.com/MedzikUser/HomeDisk.git`

### Requirements
- Rust

To build run the command: `cargo build --release`

The compiled binary can be found in `./target/release/homedisk`

## 🖴 Creating tables in a SQLite database

#### Requirements
- SQLite3

Run command `sqlite3 homedisk.db < tables.sql` to create SQLite database
