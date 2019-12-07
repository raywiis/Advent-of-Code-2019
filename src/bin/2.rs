use std::io::{prelude::*, BufReader, Result};
use std::fs::File;

fn main() -> Result<()> {
    let file = File::open("res/2_1.txt")?;
    let reader = BufReader::new(file);
    let start_state:Vec<u32> = reader.split(b',')
        .map(|instruction| instruction.unwrap())
        .map(|i| String::from_utf8(i).unwrap())
        .map(|i| i.parse::<u32>().unwrap())
        .collect();

    let mut memory = start_state.clone();

    memory = run_intcode(memory);
    println!("[Day 2] memory[0] is {}", memory[0]);

    for attempt in 0..99*99 {
        memory = start_state.clone();

        let noun = attempt / 99;
        let verb = attempt % 99;

        memory[1] = noun;
        memory[2] = verb;

        memory = run_intcode(memory);

        if memory[0] != 19_690_720 {
            continue;
        }

        let result = 100 * noun + verb;
        println!("[Day 2] noun: {}, verb: {}, result: {}", noun, verb, result);
    }

    Ok(())
}

fn run_intcode(mut memory: Vec<u32>) -> Vec<u32> {
    let mut idx = 0;

    while memory[idx] != 99 {
        let opcode = memory[idx];
        let address_a = memory[idx + 1];
        let address_b = memory[idx + 2];
        let target = memory[idx + 3];

        let a = memory[address_a as usize];
        let b = memory[address_b as usize];

        match opcode {
            1 => std::mem::replace(&mut memory[target as usize], a + b),
            2 => std::mem::replace(&mut memory[target as usize], a * b),
            _ => panic!("Unexpected opcode")
        };

        idx += 4;
    };

    memory
}
