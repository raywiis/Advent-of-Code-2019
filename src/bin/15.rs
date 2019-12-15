use advent_of_code_2019::intcode::{execute, ExitCode};
use std::collections::HashMap;
use std::cmp::{min, max};
use std::io::BufRead;
use std::io;

struct Robot {
    x: i32,
    y: i32,
    program: Vec<i64>,
    input: Vec<i64>,
    output: Vec<i64>,
}

impl Robot {
    fn new(program: Vec<i64>) -> Robot {
        Robot {
            program,
            input: vec!(),
            output: vec!(),
            x: 0,
            y: 0,
        }
    }

    fn step(&mut self, direction: char) -> char {
        match direction {
            '1' => self.y -= 1,
            '2' => self.y += 1,
            '3' => self.x -= 1,
            '4' => self.x += 1,
            _ => {
                println!("Unexpected direction .. try again");
                return 'x';
            }
        };

        self.input.push(direction.to_digit(10).unwrap_or(1) as i64);
        execute(&mut self.program.clone(), &self.input, &mut self.output);

        let tile = self.output.pop()
            .expect("No tile after stepping");
        self.output = vec!();

        match tile {
            0 => '#',
            1 => ' ',
            2 => 'O',
            _ => panic!("Wtf tile")
        }
    }

    fn backup_if_required(&mut self, tile: char, direction: char) {
        if tile != '#' {
            return;
        }

        self.back_up(direction);
    }

    fn back_up(&mut self, direction: char) {
        match direction {
            '1' => self.y += 1,
            '2' => self.y -= 1,
            '3' => self.x += 1,
            '4' => self.x -= 1,
            _ => println!("No m8"),
        };
    }

}

fn main() {
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let program = vec!(3,1033,1008,1033,1,1032,1005,1032,31,1008,1033,2,1032,1005,1032,58,1008,1033,3,1032,1005,1032,81,1008,1033,4,1032,1005,1032,104,99,1002,1034,1,1039,101,0,1036,1041,1001,1035,-1,1040,1008,1038,0,1043,102,-1,1043,1032,1,1037,1032,1042,1105,1,124,1002,1034,1,1039,101,0,1036,1041,1001,1035,1,1040,1008,1038,0,1043,1,1037,1038,1042,1105,1,124,1001,1034,-1,1039,1008,1036,0,1041,101,0,1035,1040,101,0,1038,1043,1002,1037,1,1042,1105,1,124,1001,1034,1,1039,1008,1036,0,1041,1002,1035,1,1040,102,1,1038,1043,1002,1037,1,1042,1006,1039,217,1006,1040,217,1008,1039,40,1032,1005,1032,217,1008,1040,40,1032,1005,1032,217,1008,1039,39,1032,1006,1032,165,1008,1040,39,1032,1006,1032,165,1101,0,2,1044,1105,1,224,2,1041,1043,1032,1006,1032,179,1102,1,1,1044,1106,0,224,1,1041,1043,1032,1006,1032,217,1,1042,1043,1032,1001,1032,-1,1032,1002,1032,39,1032,1,1032,1039,1032,101,-1,1032,1032,101,252,1032,211,1007,0,74,1044,1106,0,224,1101,0,0,1044,1106,0,224,1006,1044,247,102,1,1039,1034,102,1,1040,1035,1002,1041,1,1036,1002,1043,1,1038,1002,1042,1,1037,4,1044,1105,1,0,15,82,44,17,88,23,99,42,83,68,98,44,75,66,15,14,89,20,34,89,18,1,84,70,84,69,55,89,65,10,76,63,83,20,80,60,48,47,98,65,82,84,68,89,52,76,63,86,61,75,4,52,82,79,24,28,93,94,95,40,66,76,81,50,31,94,81,54,19,91,92,61,18,28,79,77,43,69,19,5,87,35,14,23,94,10,76,32,73,90,20,86,67,90,80,8,86,25,89,89,26,48,37,81,49,25,87,92,17,46,84,96,95,60,79,52,19,13,93,30,93,99,17,13,89,96,36,93,81,89,18,2,97,42,45,63,86,20,26,76,97,29,75,56,7,97,93,2,78,9,79,8,57,84,38,80,53,98,89,34,71,85,17,96,50,31,93,64,7,81,72,85,32,83,31,99,69,90,88,33,88,81,41,80,46,47,93,75,34,95,8,98,24,7,76,77,17,23,95,72,82,98,24,91,95,50,38,92,91,32,95,40,77,80,84,82,7,90,23,13,92,40,82,37,80,56,24,79,99,64,90,55,58,46,33,4,88,92,7,84,19,45,16,75,94,40,93,21,87,94,79,39,83,52,92,14,21,77,82,5,84,85,48,75,19,26,91,28,99,87,81,86,24,53,98,52,25,2,75,39,82,24,51,77,47,92,53,94,27,34,85,22,25,36,92,79,29,2,10,19,95,13,96,82,56,99,3,91,62,99,43,49,7,91,96,77,89,7,99,86,24,92,57,24,49,3,96,77,35,75,11,86,21,1,82,67,84,90,75,96,9,83,1,47,78,7,98,30,11,88,52,78,58,98,47,90,46,78,14,77,88,3,97,87,70,75,24,98,5,80,87,93,95,22,37,59,85,23,41,89,91,9,7,90,61,3,95,96,92,25,57,47,38,88,14,15,84,31,79,20,79,77,22,33,90,70,89,78,51,24,93,81,21,79,82,17,75,88,78,26,87,24,38,96,50,81,6,46,93,39,91,92,81,39,91,5,79,58,9,87,50,83,63,87,2,29,92,37,81,55,59,99,91,35,9,96,18,82,66,4,89,44,87,92,6,79,88,9,9,63,88,71,77,91,35,29,87,87,51,20,94,19,57,93,72,89,4,77,10,87,20,67,80,79,71,1,75,28,87,88,87,55,37,80,85,5,55,5,97,12,62,88,82,27,6,99,93,42,91,16,75,80,6,20,96,6,84,6,46,84,23,92,93,32,90,79,3,54,7,97,92,92,33,79,9,5,10,90,76,19,76,1,85,83,58,2,91,83,77,59,63,89,26,97,67,96,52,88,62,65,23,91,94,51,31,80,24,5,72,40,81,9,85,79,12,98,44,45,81,25,30,60,5,76,92,62,18,32,78,25,16,76,97,18,96,39,96,60,78,78,47,99,48,82,98,57,96,98,73,89,18,12,91,8,66,85,57,94,22,76,88,98,39,58,96,91,61,98,89,7,77,91,13,96,20,86,2,88,91,27,75,32,29,79,51,81,4,86,10,37,79,84,67,49,75,20,94,91,23,33,92,38,91,37,76,79,55,91,43,80,25,98,77,91,88,44,15,97,45,3,86,73,87,30,91,62,80,80,16,85,54,88,54,75,88,65,18,85,22,90,79,36,10,77,86,65,30,38,85,3,90,44,48,75,81,80,32,59,90,91,41,95,72,79,11,66,26,96,20,4,68,88,23,95,31,98,12,98,56,94,95,80,68,78,39,79,93,85,55,96,4,77,14,80,46,95,84,84,6,93,35,95,46,85,92,81,69,85,92,87,0,0,21,21,1,10,1,0,0,0,0,0,0);
    let mut robot = Robot::new(program);

    map.insert((0,0), '.');

    let stdin = io::stdin();
    let mut stdin_lines = stdin.lock().lines();

    let mut oxygen_location = (0, 0);
    let mut depth = 0;
    while true {
        // let direction = stdin_lines.next()
        //     .unwrap_or(Ok("1".to_owned())).unwrap()
        //     .chars().next().unwrap();
        let saved = map.insert((robot.x, robot.y), '@')
            .expect("Oops");
        // println!("{}", saved);
        // show_map(&map);
        map.insert((robot.x, robot.y), saved);

        // std::thread::sleep_ms(1);
        let mut direction = '0';
        for i in 1..5 {
            if is_visited(&map, robot.x, robot.y, i) {
                continue;
            }
            direction = std::char::from_digit(i as u32, 10)
                .expect("failed parsing things");
            break;
        }
        // println!("Direction {}", direction);

        if direction == '0' {
            // println!("{:?}", robot.input);
            let last_move = robot.input.pop();
            if last_move == None {
                break;
            }
            // println!("{:?}", robot.input);
            robot.back_up(std::char::from_digit(last_move.unwrap() as u32, 10)
                .expect("failed parsing other things"));
            depth -= 1;
            continue;
        }

        depth += 1;
        let tile = robot.step(direction);
        if tile == 'O' {
            println!("Oxygen in {} steps", depth);
            oxygen_location = (robot.x, robot.y);
            // return;
        }

        map.insert((robot.x, robot.y), tile);
        if tile == '#' {
            depth -= 1;
            robot.input.pop();
        }
        robot.backup_if_required(
            tile,direction
        );

        // for i in 1..5 {
        //     probe(&mut robot, std::char::from_digit(i, 10).expect("failed parsing 1..5"), &mut map);
        // }
    }

    // show_map(&map);
    println!("{:?}", oxygen_location);

    let mut front: Vec<(i32, i32)> = vec!(oxygen_location);
    let mut wave = vec!();
    let mut wave_count = -1;
    while front.len() != 0 {
        wave = front.clone();
        front = vec!();
        wave_count += 1;

        for item in wave {
            map.insert(item, '0');

            if *map.get(&(item.0 + 1, item.1)).unwrap() == ' ' {
                front.push((item.0 + 1, item.1));
            }
            if *map.get(&(item.0 - 1, item.1)).unwrap() == ' ' {
                front.push((item.0 - 1, item.1));
            }
            if *map.get(&(item.0, item.1 + 1)).unwrap() == ' ' {
                front.push((item.0, item.1 + 1));
            }
            if *map.get(&(item.0, item.1 - 1)).unwrap() == ' ' {
                front.push((item.0, item.1 - 1));
            }
        }

        // show_map(&map);
        // std::thread::sleep_ms(200);
        // println!("{} minutes", wave_count);
    }

    println!("Minutes to fill with oxygen {}", wave_count);
}

fn is_visited(map: &HashMap<(i32, i32), char>, x: i32, y: i32, direction: i32) -> bool {
    let mut tile = (x, y);

    match direction {
        1 => tile.1 -= 1,
        2 => tile.1 += 1,
        3 => tile.0 -= 1,
        4 => tile.0 += 1,
        _ => panic!("Not sure if visited"),
    };
    // println!("tile {:?}", tile);

    match map.get(&tile) {
        Some(_) => true,
        None => false,
    }
}

fn probe(robot: &mut Robot, direction: char, map: &mut HashMap<(i32, i32), char>) -> char {
    let tile = robot.step(direction);
    map.insert((robot.x, robot.y), tile);
    robot.back_up(direction);
    robot.input.pop();
    tile
}

fn show_map(map: &HashMap<(i32, i32), char>) {
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;

    for (k, v) in map.iter() {
        x_min = min(x_min, k.0);
        x_max = max(x_max, k.0);
        y_min = min(y_min, k.1);
        y_max = max(y_max, k.1);
    }

    // println!("{:?}", map);
    // println!("{} {} {} {}", x_min, x_max, y_min, y_max);

    let x_delta = x_max - x_min;
    let y_delta = y_max - y_min;

    for y in 0..y_delta+1 {
        for x in 0..x_delta+1 {
            let tile_opt = map.get(&(x + x_min, y + y_min));
            match tile_opt {
                Some(tile) => print!("{}", tile),
                None => print!(".")
            }
        }
        println!();
    }
}
