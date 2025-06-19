use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Instruction {
    Push(i32),
    Add,
    Sub,
    Mul,
    Div,
    Print,
    Halt,
}

fn parse_line(line: &str) -> Option<Instruction> {
    let tokens: Vec<&str> = line.trim().split_whitespace().collect();
    match tokens.as_slice() {
        ["PUSH", val] => val.parse().ok().map(Instruction::Push),
        ["ADD"] => Some(Instruction::Add),
        ["SUB"] => Some(Instruction::Sub),
        ["MUL"] => Some(Instruction::Mul),
        ["DIV"] => Some(Instruction::Div),
        ["PRINT"] => Some(Instruction::Print),
        ["HALT"] => Some(Instruction::Halt),
        _ => None,
    }
}

fn execute(program: &[Instruction]) {
    let mut stack: Vec<i32> = Vec::new();

    for instr in program {
        match instr {
            Instruction::Push(val) => stack.push(*val),
            Instruction::Add => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a + b);
                }
            }
            Instruction::Sub => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a - b);
                }
            }
            Instruction::Mul => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a * b);
                }
            }
            Instruction::Div => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a / b);
                }
            }
            Instruction::Print => {
                if let Some(val) = stack.last() {
                    println!("Top of Stack: {}", val);
                }
            }
            Instruction::Halt => {
                println!("Program halted.");
                break;
            }
        }
    }
}

fn main() -> io::Result<()> {
    let path = "bytecode.txt";
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut program: Vec<Instruction> = Vec::new();

    for line in reader.lines() {
        if let Ok(instruction_line) = line {
            if let Some(instr) = parse_line(&instruction_line) {
                program.push(instr);
            }
        }
    }

    execute(&program);
    Ok(())
}


