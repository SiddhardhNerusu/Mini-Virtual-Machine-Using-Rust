# Mini Bytecode Interpreter in Rust

This is a lightweight stack-based virtual machine written in Rust. It reads custom bytecode instructions from a file and executes them sequentially.

## Supported Instructions
- `PUSH <int>`: Push an integer to the stack
- `ADD`: Add top two stack values
- `SUB`: Subtract top two stack values
- `MUL`: Multiply top two stack values
- `DIV`: Divide top two stack values
- `PRINT`: Print top of stack
- `HALT`: Stop execution

## Files
- `src/main.rs`: Rust implementation
- `bytecode.txt`: Sample bytecode input

## Run It
```bash
rustc src/main.rs
./main
