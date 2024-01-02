# Shellcode Template

## Overview
This Rust-based tool translates assembly instructions into raw and Python-escaped opcodes.

## Features
- Translates assembly instructions to machine code opcodes.
- Outputs raw byte representation of the opcodes.
- Generates Python-escaped string for the opcodes.
- Provides disassembly output for verification and readability.

## Requirements
- Rust
- Keystone Engine (for assembly)

## Installation
1. Ensure Rust is installed on your system.
2. Install Keystone. On Ubuntu, for example:
```bash
sudo apt-get install libkeystone-dev
```

```sh
paru -S keystone
```

## Usage
Add your ASM code inside the `code` variable and:
```sh
cargo run
```
