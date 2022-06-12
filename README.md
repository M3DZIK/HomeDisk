# HomeDisk cloud server

[![docs-rs]](https://homedisk-doc.vercel.app)
[![total-lines]](https://github.com/MedzikUser/HomeDisk)
[![code-size]](https://github.com/MedzikUser/HomeDisk)
[![CI]](https://github.com/MedzikUser/HomeDisk/actions/workflows/rust.yml)

[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
[total-lines]: https://img.shields.io/tokei/lines/github/MedzikUser/HomeDisk?style=for-the-badge&logo=github&color=fede00
[code-size]: https://img.shields.io/github/languages/code-size/MedzikUser/HomeDisk?style=for-the-badge&color=c8df52&logo=github
[CI]: https://img.shields.io/github/workflow/status/MedzikUser/rust-crypto-utils/Rust/main?style=for-the-badge

![](https://i.imgur.com/fOtiSf7.png)

![](https://i.imgur.com/vLautmq.png)

## 👨‍💻 Building

First clone the repository: `git clone git@github.com:MedzikUser/HomeDisk.git`

### Server

#### Requirements
- Rust

To build run the command: `cargo build --release`

The compiled binary can be found in `./target/release/cloud`

### Website

#### Requirements
- Node.js
- pnpm

Run these commands to build:

- Go to directory `./website`
- Install dependencies: `pnpm install`
- Build website: `pnpm run build`
- Export website to static HTML files: `pnpm run export` (Optional)

If you exported the page to HTML files, they are located in the `./out` directory,
if not, you can start the site with `pnpm run start`

## 🖴 Creating tables in a SQLite database

#### Requirements
- SQLite3

Run command `sqlite3 homedisk.db < tables.sql` to create SQLite database
