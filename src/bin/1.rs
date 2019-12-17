use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::cmp;

fn main() {
    let input = read_input("res/1_1.txt");
    let cargo_fuel = input.iter()
        .map(|w| get_fuel_cost(*w, false))
        .sum::<i32>();
    
    let total_fuel = input.iter()
        .map(|w| get_fuel_cost(*w, true))
        .sum::<i32>();

    println!("[Day 1] Cargo fuel cost: {}", cargo_fuel);
    println!("[Day 1] Total fuel cost: {}", total_fuel);
}

fn get_fuel_cost(mass: i32, recurse: bool) -> i32 {
    let cargo_cost = cmp::max((mass / 3) - 2, 0);
    if cargo_cost == 0 || !recurse {
        cargo_cost
    } else {
        cargo_cost + get_fuel_cost(cargo_cost, true)
    }
}

fn read_input(filename: &str) -> Vec<i32> {
    let file = File::open(filename)
        .expect("Failed to open file");
    let reader = BufReader::new(file);

    let total_weight = reader.lines()
        .map(|r| r.expect("Failed to read line"))
        .map(|l| l.to_string())
        .map(|s| s.parse::<i32>()
            .expect("Failed to parse int"))
        .collect();
    
    total_weight
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_samples() {
        assert_eq!(get_fuel_cost(12, false), 2);
        assert_eq!(get_fuel_cost(14, false), 2);
        assert_eq!(get_fuel_cost(1969, false), 654);
        assert_eq!(get_fuel_cost(100756, false), 33583);
    }

    #[test]
    fn part_1() {
        let fuel_sum = read_input("res/1_1.txt").iter()
            .map(|w| get_fuel_cost(*w, false))
            .sum::<i32>();
        assert_eq!(fuel_sum, 3457281);
    }

    #[test]
    fn part_2_samples() {
        assert_eq!(get_fuel_cost(14, true), 2);
        assert_eq!(get_fuel_cost(1969, true), 966);
        assert_eq!(get_fuel_cost(100756, true), 50346);
    }

    #[test]
    fn part_2() {
        let fuel_sum = read_input("res/1_1.txt").iter()
            .map(|w| get_fuel_cost(*w, true))
            .sum::<i32>();
        assert_eq!(fuel_sum, 5183030);
    }
}
