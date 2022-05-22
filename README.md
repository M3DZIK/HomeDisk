# HomeDisk cloud server

![](https://i.imgur.com/gi7KBVE.png)

![](https://i.imgur.com/vLautmq.png)

## 👨‍💻 Building

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
