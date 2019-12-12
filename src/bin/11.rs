use std::collections::HashMap;
use advent_of_code_2019::intcode::{execute, ExitCode};
use std::cmp::{max, min};

struct Robot {
    program: Vec<i64>,
    inputs: Vec<i64>,
    position_x: i64,
    position_y: i64,
    direction: i32,
    map: HashMap<(i64, i64), i32>,
    last_idx: usize
}

impl Robot {
    fn new(program: Vec<i64>) -> Robot {
        let inputs = vec!(1);
        let mut map = HashMap::new();
        map.insert((0, 0), 1);
        Robot{ 
            program,
            inputs,
            position_x: 0,
            position_y: 0,
            direction: 1,
            map,
            last_idx: 0
        }
    }

    fn cycle(&mut self) -> ExitCode {
        let mut outputs = vec!();
        let exit = execute(
            &mut self.program.clone(),
            &self.inputs,
            &mut outputs
        );


        let idx = outputs.len();
        let color = outputs[idx - 2];
        let turn = outputs[idx - 1];

        if self.last_idx + 2 != idx {
            panic!("Robot commands skipped");
        }
        self.last_idx = idx;

        self.paint(color as i32);
        self.turn(turn as i32);
        self.step();

        let stand_color = self.get_color();
        self.inputs.push(stand_color);

        exit
    }

    fn get_color(&self) -> i64 {
        let x = self.position_x;
        let y = self.position_y;

        match self.map.get(&(x, y)) {
            Some(color) => *color as i64,
            None => 0
        }
    }

    fn turn(&mut self, direction: i32) {
        match direction {
            // Left
            0 => self.direction = (self.direction + 3) % 4,
            // Right
            1 => self.direction = (self.direction + 1) % 4,
            _ => panic!("Unexpected turn")
        }
    }

    fn step(&mut self) {
        match self.direction {
            0 => self.position_x -= 1,
            1 => self.position_y += 1,
            2 => self.position_x += 1,
            3 => self.position_y -= 1,
            _ => panic!("Unexpected direction")
        }
    }

    fn paint(&mut self, color: i32) {
        let x = self.position_x;
        let y = self.position_y;
        self.map.insert((x, y), color);
    }
}

fn main() {
    let program = vec!(3,8,1005,8,304,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,1002,8,1,29,2,103,1,10,1,106,18,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,102,1,8,59,2,102,3,10,2,1101,12,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,88,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,101,0,8,110,2,108,9,10,1006,0,56,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,139,1,108,20,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,0,8,10,4,10,102,1,8,165,1,104,9,10,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,1001,8,0,192,2,9,14,10,2,1103,5,10,1,1108,5,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,102,1,8,226,1006,0,73,1006,0,20,1,1106,11,10,1,1105,7,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,1001,8,0,261,3,8,102,-1,8,10,101,1,10,10,4,10,108,1,8,10,4,10,1002,8,1,283,101,1,9,9,1007,9,1052,10,1005,10,15,99,109,626,104,0,104,1,21101,48062899092,0,1,21101,0,321,0,1105,1,425,21101,936995300108,0,1,21101,0,332,0,1106,0,425,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,209382902951,1,1,21101,379,0,0,1106,0,425,21102,179544747200,1,1,21102,390,1,0,1106,0,425,3,10,104,0,104,0,3,10,104,0,104,0,21102,1,709488292628,1,21102,1,413,0,1106,0,425,21101,0,983929868648,1,21101,424,0,0,1105,1,425,99,109,2,22101,0,-1,1,21102,40,1,2,21102,456,1,3,21101,446,0,0,1106,0,489,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,451,452,467,4,0,1001,451,1,451,108,4,451,10,1006,10,483,1102,0,1,451,109,-2,2105,1,0,0,109,4,1201,-1,0,488,1207,-3,0,10,1006,10,506,21102,1,0,-3,21202,-3,1,1,21201,-2,0,2,21101,0,1,3,21101,525,0,0,1105,1,530,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,553,2207,-4,-2,10,1006,10,553,21202,-4,1,-4,1105,1,621,21201,-4,0,1,21201,-3,-1,2,21202,-2,2,3,21102,1,572,0,1106,0,530,21201,1,0,-4,21101,0,1,-1,2207,-4,-2,10,1006,10,591,21102,0,1,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,613,22101,0,-1,1,21101,0,613,0,106,0,488,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2106,0,0);

    let mut robot = Robot::new(program);
    let mut exit_code = ExitCode::NoInput;

    while exit_code == ExitCode::NoInput {
        exit_code = robot.cycle();
    }

    println!("Done: {}", robot.map.len());

    let pixels: Vec<((i64, i64), i32)> = robot.map.iter()
        .map(|(key, val)| ((key.0, key.1), *val))
        .collect();

    let max_x = pixels.iter().map(|val| (val.0).0).fold(0, max);
    let min_x = pixels.iter().map(|val| (val.0).0).fold(max_x, min);
    let max_y = pixels.iter().map(|val| (val.0).1).fold(-100, max);
    let min_y = pixels.iter().map(|val| (val.0).1).fold(100, min);

    println!("{}, {}, {}, {}", max_x, min_x, max_y, min_y);

    let mut grid: [[char; 43]; 6] = [['.'; 43]; 6];

    for (position, val) in pixels {
        if val == 1 {
            grid[position.1.abs() as usize][position.0 as usize] = '#';
        } else {
            grid[position.1.abs() as usize][position.0 as usize] = '.';
        }
    }

    for line in grid.iter() {
        for pixel in line.iter() {
            print!("{}", pixel);
        }
        println!();
    }
}
