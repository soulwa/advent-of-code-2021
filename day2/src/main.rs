use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32)
}

impl FromStr for Instruction { 
    type Err = String;

    // assume form "instr <val>"
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> { 
        let s: Vec<_> = s.split(" ").collect();
        let instr = s[0];
        let val: i32 = s[1].parse().unwrap();

        match instr {
            "forward" => Ok(Instruction::Forward(val)),
            "down" => Ok(Instruction::Down(val)),
            "up" => Ok(Instruction::Up(val)),
            _ => Err("Invalid instructiont type!".to_string())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(File::open("input")?)
        .lines()
        .map(|l| Instruction::from_str(&l.unwrap()).unwrap())
        .collect::<Vec<_>>();

    let mut depth = 0;
    let mut dst = 0;

    // part 1
    for instr in &input {
        match instr {
            Instruction::Forward(val) => dst += val,
            Instruction::Down(val) => depth += val,
            Instruction::Up(val) => depth -= val,
        }
    }

    // part 2
    depth = 0;
    dst = 0;
    let mut aim = 0;
    for instr in input {
        match instr {
            Instruction::Forward(val) => {
                dst += val;
                depth += val * aim;
            },
            Instruction::Down(val) => aim += val,
            Instruction::Up(val) => aim -= val,
        }
    }

    println!("Final depth: {}", depth);
    println!("Final distance: {}", dst);
    println!("Final value: {}", depth * dst);

    Ok(())
}