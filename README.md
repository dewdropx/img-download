# Image Downloader in Rust

A simple Rust project demonstrating how to download images from the internet using HTTP requests, handle errors, and save binary data to files.

## Overview

This project contains two implementations of an image downloader:

- **Permanent Download:** Downloads an image and saves it in the current working directory.
- **Temporary Download:** Downloads an image into a temporary directory that is automatically deleted when the directory's `TempDir` handle is dropped.

The project also demonstrates asynchronous programming with Tokio, HTTP requests with Reqwest, error handling with `error-chain`, and file operations using Rust's standard library.

## Technologies Used

- **Rust** — Programming language
- **Reqwest** — HTTP requests and response handling
- **Tokio** — Asynchronous runtime
- **Tempfile** — Temporary directory management
- **Error Chain** — Error handling and propagation

## Project Structure

```text
img-download/
├── Cargo.toml
├── Cargo.lock
└── src/
    ├── main.rs
    └── bin/
        ├── permanent_download.rs
        └── temp_download.rs
```

## Prerequisites

Install Rust and Cargo using [rustup](https://rustup.rs/).

Verify your installation:

```bash
rustc --version
cargo --version
```

## How to Run

Clone the repository and navigate to the project directory.

### 1. Run the default program

```bash
cargo run
```

Runs the Hello World program in `src/main.rs`.

### 2. Permanent Image Download

```bash
cargo run --bin permanent_download
```

Downloads the Rust logo and saves it as `rust-logo-512x512.png` in the current working directory.

The file remains available after the program finishes.

### 3. Temporary Image Download

```bash
cargo run --bin temp_download
```

Downloads the Rust logo into a uniquely named temporary directory.

The directory and its contents are automatically cleaned up when the `TempDir` value is dropped, typically when the program exits.

## Dependencies

The project uses the following crates:

```toml
[dependencies]
tempfile = "3.3.0"
error-chain = "0.12.4"
reqwest = "0.13"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

Cargo resolves and downloads the dependencies automatically when you run the project.
