<p align="center">
    <img width="500" src="https://raw.githubusercontent.com/HomeDisk/.github/main/img/HomeDisk.svg" alt="HomeDisk Icon" />
</p>

<p align="center">
    <a href="https://github.com/MedzikUser/HomeDisk"><img src="https://img.shields.io/badge/built_with-Rust-dca282.svg?style=flat-square"></a>
    &nbsp;
    <a href="https://github.com/MedzikUser/HomeDisk"><img src="https://img.shields.io/badge/license-GPL_3.0-00bfff.svg?style=flat-square"></a>
    &nbsp;
    <a href="https://github.com/MedzikUser/HomeDisk"><img src="https://img.shields.io/github/workflow/status/MedzikUser/HomeDisk/Rust/main?style=flat-square"></a>
    &nbsp;
    <a href="https://homedisk-doc.vercel.app"><img src="https://img.shields.io/badge/docs.rs-66c2a5?style=flat-square&labelColor=555555&logo=docs.rs"></a>
    &nbsp;
    <a href="https://documenter.getpostman.com/view/23280189/VVk9dwRk"><img src="https://img.shields.io/badge/API_Docs-887BB0?style=flat-square&labelColor=555555&logo=postman"></a>
</p>

## Documentation

### 👨‍💻 Compile server

```bash
cargo build --release
```

Now you can run server using command `./target/release/homedisk`.

### 🔒 Generate development TLS certificate

```bash
# Generate private key
openssl genrsa -out cert.key 204
# Generate certificate
openssl req -new -x509 -key cert.key -out cert.pem -days 365
```
