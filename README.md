# GitAuto

**GitAuto** is a fast and secure CLI tool built in Rust that automatically generates Conventional Commit messages using local AI models.

![Project Status: Beta](https://img.shields.io/badge/Status-Beta-yellow)
![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange)

# 🎥 Video preview

![Preview video](docs/demo.gif)

## 🚀 How to build

### Prerequisites

To compile the project, you will need the [Rust toolchain and Cargo package manager](https://doc.rust-lang.org/cargo) installed on your system.

### Building from Source

Clone the repository and build the project in release mode:

```bash
git clone [https://github.com/Darkx32/GitAuto.git](https://github.com/Darkx32/GitAuto.git)
cd GitAuto
cargo build --release
```

# 📝 TO-DO

- [X] Fully configurable and auto install model to use
- [X] Generate commits using AI
- [ ] Better performance on specified backends (CUDA, MKL, METAL, ...)
- [ ] Add new models