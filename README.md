# 🦀 Mini Light EVM

A lightweight, educational Ethereum Virtual Machine (EVM) implemented in Rust. This project aims to demonstrate how Ethereum opcodes work and how transactions are executed with bytecode at a low level.

> ⚠️ **Note:** This project is still in progress and is not intended for production use. Contributions and feedback are welcome!

---

## 🚀 Motivation

The Ethereum Virtual Machine is the heart of Ethereum smart contract execution. However, understanding its inner workings can be daunting. This project serves as a simplified version of the EVM to:

- Learn how opcodes are parsed and executed
- Explore transaction execution flow
- Build intuition around EVM bytecode and stack-based execution
- Serve as a teaching tool or experimental playground

---

## 🧱 Features (so far)

- [x] Stack-based virtual machine
- [x] Basic opcode parsing and execution (`PUSH`, `ADD`, `MUL`, etc.)
- [ ] Storage and memory simulation
- [ ] Gas metering


---

## 📦 Installation

Ensure you have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 📦 Clone Project

```
git clone https://github.com/shaaibu7/REVM.git
cd REVM
cargo build

