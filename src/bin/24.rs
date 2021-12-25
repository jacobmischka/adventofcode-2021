use adventofcode_2021::get_input;

use std::{collections::HashMap, str::FromStr};

fn main() {
    let s = get_input().unwrap();
    let instructions: Vec<Instruction> = s
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();

    let mut alu = ALU {
        instructions,
        input_buffer: vec![9; 14],
        ..Default::default()
    };

    let mut locked_in = vec![false; 14];
    let mut prev_prev;
    let mut prev = vec![0; 14];
    let mut goal = 11000;

    loop {
        alu.reset();
        dec_vec(&mut alu.input_buffer, &mut locked_in);

        if let Ok(0) = alu.run() {
            println!(
                "Part 1: {}",
                alu.input_buffer
                    .iter()
                    .copied()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            );
            break;
        }

        if goal < 15 {
            // dbg!(&alu.input_buffer);
        }

        if alu.vars.get(&'z').copied().unwrap_or_default() < goal {
            prev_prev = prev;
            prev = alu.input_buffer.clone();

            for i in (0..(prev.len() - 2)).rev() {
                if locked_in[i] {
                    continue;
                }

                if prev[i] == prev_prev[i] && prev[i] == alu.input_buffer[i] {
                    goal /= 2;
                    locked_in[i] = true;
                    dbg!(&locked_in);
                    break;
                }
            }

            dbg!(&alu.input_buffer, &alu.vars, goal);
            eprintln!();
        }
    }
}

fn dec_vec(v: &mut [i64], locked: &mut [bool]) {
    let mut should_dec = false;
    let len = v.len() - 1;
    let mut i = len;

    if v[i] > 6 {
        v[i] = 6;
    }

    if v[i - 1] == v[i] - 1 {
        v[i] -= 1;
        if v[i] <= 4 {
            v[i] = 6;
            should_dec = true;
        }
    }

    v[i - 1] = v[i] - 1;
    v[i - 2] = v[i] + 3;
    // v[i - 3] = v[i] - 3;

    // if v[4] > 3 {
    //     v[4] = 3;
    // }

    if should_dec {
        i -= 3;
        loop {
            if i == 0 {
                for j in (0..(locked.len())).rev() {
                    if locked[j] {
                        locked[j] = false;
                        return;
                    }
                }
            }

            if locked[i] {
                i -= 1;
            } else if v[i] == 1 {
                v[i] = 9;
                i -= 1;
            } else {
                v[i] -= 1;
                break;
            }
        }
    }
}

fn inc_vec(v: &mut Vec<i64>) {
    let mut i = v.len() - 1;
    loop {
        if v[i] == 9 {
            v[i] = 1;
            i -= 1;
        } else {
            v[i] += 1;
            break;
        }
    }
}

type Variable = char;

#[derive(Debug, Clone, Copy)]
enum Operand {
    Literal(i64),
    Variable(Variable),
}

impl FromStr for Operand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().next().unwrap();
        match c {
            'a'..='z' => {
                assert_eq!(s.len(), 1);
                Ok(Operand::Variable(c))
            }
            _ => Ok(Operand::Literal(s.parse::<i64>().unwrap())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Inp(Variable),
    Add(Variable, Operand),
    Mul(Variable, Operand),
    Div(Variable, Operand),
    Mod(Variable, Operand),
    Eql(Variable, Operand),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split_whitespace();
        let inst = pieces.next().unwrap();

        let var = pieces.next().unwrap();
        assert_eq!(var.len(), 1);
        let var = var.chars().next().unwrap();

        if inst == "inp" {
            return Ok(Instruction::Inp(var));
        }

        let op = Operand::from_str(pieces.next().unwrap()).unwrap();

        match inst {
            "add" => Ok(Instruction::Add(var, op)),
            "mul" => Ok(Instruction::Mul(var, op)),
            "div" => Ok(Instruction::Div(var, op)),
            "mod" => Ok(Instruction::Mod(var, op)),
            "eql" => Ok(Instruction::Eql(var, op)),
            _ => Err(format!("invalid instruction: {}", inst)),
        }
    }
}

#[derive(Debug, Default)]
struct ALU {
    vars: HashMap<Variable, i64>,
    instructions: Vec<Instruction>,
    pc: usize,
    input_buffer: Vec<i64>,
    ic: usize,
}

impl ALU {
    fn reset(&mut self) {
        self.vars.clear();
        self.pc = 0;
        self.ic = 0;
    }

    fn run(&mut self) -> Result<i64, Error> {
        while self.pc < self.instructions.len() {
            self.step()?;
        }

        Ok(self.vars.get(&'z').copied().unwrap_or_default())
    }

    fn get_value(&self, op: &Operand) -> i64 {
        match op {
            Operand::Literal(val) => *val,
            Operand::Variable(var) => self.vars.get(var).copied().unwrap_or_default(),
        }
    }

    fn step(&mut self) -> Result<(), Error> {
        match self.instructions[self.pc] {
            Instruction::Inp(var) => {
                self.vars.insert(var, self.input_buffer[self.ic]);
                self.ic += 1;
            }
            Instruction::Add(var, op) => {
                let rhs = self.get_value(&op);
                *self.vars.entry(var).or_default() += rhs;
            }
            Instruction::Mul(var, op) => {
                let rhs = self.get_value(&op);
                *self.vars.entry(var).or_default() *= rhs;
            }
            Instruction::Div(var, op) => {
                let rhs = self.get_value(&op);
                if rhs == 0 {
                    return Err(Error::DivByZero);
                }
                *self.vars.entry(var).or_default() /= rhs;
            }
            Instruction::Mod(var, op) => {
                let rhs = self.get_value(&op);
                if rhs == 0 {
                    return Err(Error::DivByZero);
                }
                *self.vars.entry(var).or_default() %= rhs;
            }
            Instruction::Eql(var, op) => {
                let lhs = self.vars.get(&var).copied().unwrap_or_default();
                let rhs = self.get_value(&op);
                self.vars.insert(var, if lhs == rhs { 1 } else { 0 });
            }
        }

        self.pc += 1;

        Ok(())
    }
}

enum Error {
    DivByZero,
}
