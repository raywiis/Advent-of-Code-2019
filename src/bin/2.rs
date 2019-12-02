use std::io::{prelude::*, BufReader, Result};
use std::fs::File;

fn main() -> Result<()> {
    let file = File::open("res/2_1.txt")?;
    let reader = BufReader::new(file);
    let mut memory:Vec<u32> = reader.split(',' as u8)
        .map(|instruction| instruction.unwrap())
        .map(|i| String::from_utf8(i).expect("Failed to parse string"))
        .map(|i| i.parse::<u32>().expect("Failed to parse u8"))
        .collect();
    let starting_memory = memory.clone();

    for attempt in 0..99*99 {
        let noun = attempt / 99;
        let verb = attempt % 99;

        memory[1] = noun;
        memory[2] = verb;

        let mut idx = 0;
        while memory[idx] != 99 {
            let opcode = memory[idx];
            let pos_a = memory[idx + 1];
            let pos_b = memory[idx + 2];
            let target = memory[idx + 3];

            let a = memory[pos_a as usize];
            let b = memory[pos_b as usize];

            match opcode {
                1 => std::mem::replace(&mut memory[target as usize], a + b),
                2 => std::mem::replace(&mut memory[target as usize], a * b),
                _ => panic!("Unexpected opcode")
            };

            idx += 4;
        };

        if memory[0] == 19690720 {
            let result = 100 * noun + verb;
            println!("Noun: {}, Verb: {}, Result: {}", noun, verb, result);
            return Ok(());
        }

        memory = starting_memory.clone();
    }

    println!("After running memory[0] is {}", memory[0]);
    Ok(())
}
