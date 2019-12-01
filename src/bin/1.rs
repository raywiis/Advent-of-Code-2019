use std::fs::File;
use std::io::{prelude::*, BufReader, Result};
use std::cmp;

fn get_fuel_cost(mass: i32) -> i32 {
    let cargo_cost = cmp::max((mass / 3) - 2, 0);
    if cargo_cost == 0 {
        cargo_cost
    } else {
        cargo_cost + get_fuel_cost(cargo_cost)
    }
}

fn main() -> Result<()> {
    let file = File::open("res/1_1.txt")?;
    let reader = BufReader::new(file);
    let mut fuel_sum = 0;

    for line in reader.lines() {
        let weight = line?.to_string()
            .parse::<i32>().expect("Failed to parse");
        let cost = get_fuel_cost(weight);
        fuel_sum += cost;
    }


    println!("[Day 1] Total fuel cost: {}", fuel_sum);
    Ok(())
}
