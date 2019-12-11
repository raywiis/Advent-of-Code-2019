use advent_of_code_2019::intcode::{execute, ExitCode};
use std::collections::HashMap;

#[derive(Hash, Clone, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

fn main() {
    let mut program = vec!(3,8,1005,8,304,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,1002,8,1,29,2,103,1,10,1,106,18,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,102,1,8,59,2,102,3,10,2,1101,12,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,88,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,101,0,8,110,2,108,9,10,1006,0,56,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,139,1,108,20,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,0,8,10,4,10,102,1,8,165,1,104,9,10,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,1001,8,0,192,2,9,14,10,2,1103,5,10,1,1108,5,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,102,1,8,226,1006,0,73,1006,0,20,1,1106,11,10,1,1105,7,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,1001,8,0,261,3,8,102,-1,8,10,101,1,10,10,4,10,108,1,8,10,4,10,1002,8,1,283,101,1,9,9,1007,9,1052,10,1005,10,15,99,109,626,104,0,104,1,21101,48062899092,0,1,21101,0,321,0,1105,1,425,21101,936995300108,0,1,21101,0,332,0,1106,0,425,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,209382902951,1,1,21101,379,0,0,1106,0,425,21102,179544747200,1,1,21102,390,1,0,1106,0,425,3,10,104,0,104,0,3,10,104,0,104,0,21102,1,709488292628,1,21102,1,413,0,1106,0,425,21101,0,983929868648,1,21101,424,0,0,1105,1,425,99,109,2,22101,0,-1,1,21102,40,1,2,21102,456,1,3,21101,446,0,0,1106,0,489,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,451,452,467,4,0,1001,451,1,451,108,4,451,10,1006,10,483,1102,0,1,451,109,-2,2105,1,0,0,109,4,1201,-1,0,488,1207,-3,0,10,1006,10,506,21102,1,0,-3,21202,-3,1,1,21201,-2,0,2,21101,0,1,3,21101,525,0,0,1105,1,530,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,553,2207,-4,-2,10,1006,10,553,21202,-4,1,-4,1105,1,621,21201,-4,0,1,21201,-3,-1,2,21202,-2,2,3,21102,1,572,0,1106,0,530,21201,1,0,-4,21101,0,1,-1,2207,-4,-2,10,1006,10,591,21102,0,1,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,613,22101,0,-1,1,21101,0,613,0,106,0,488,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2106,0,0);
    let mut input = vec!();
    let mut output = vec!();

    let mut wall: HashMap<Point, i32> = HashMap::new();

    let mut exitCode = ExitCode::NoInput;
    let mut position = Point{ x: 0, y: 0 };
    let mut direction = 1;
    let mut iter = 0;
    while exitCode == ExitCode::NoInput {
        let new_input = (get_color(&wall, &position) as i64);
        input.push(new_input);
        output = vec!();

        exitCode = mock_execute(&mut program, &input, &mut output);

        let idx = output.len();
        let turn = output[idx - 1];
        let color = output[idx - 2];

        set_color(&mut wall, position.clone(), color as i32);

        let res = move_robot(position, direction, turn);
        position = res.0;
        direction = res.1;


        // println!("{:?}", input);
        // println!("{}, {}, {:?}: {:?}", color, direction, position, (turn, color));
        // println!("{:?}", wall);
        // iter += 1;
        // if iter == 1000 {
        //     exitCode = ExitCode::Ok
        // }
    }

    println!("Walls painted {}", wall.len());
}

fn mock_execute(
    memory: &mut Vec<i64>,
    inputs: &[i64],
    outputs: &mut Vec<i64>
) -> ExitCode {
    outputs.append(&mut vec!(1, 0, 0, 0));
    ExitCode::Ok
}

fn move_robot(start: Point, direction: i32, turn: i64) -> (Point, i32) {
    let mut next: Point = Point{ x: start.x, y: start.y };

    // println!("{:?}, {}, {}", start, direction, turn);

    let mut next_direction = match turn {
        0 => (direction + 3) % 4,
        1 => (direction + 1) % 4,
        _ => panic!("Destroyed")
    };

    match next_direction {
        0 => next.x -= 1,
        1 => next.y += 1,
        2 => next.x += 1,
        3 => next.y -= 1,
        _ => panic!("What in tarnation")
    }

    // println!("{}, {:?}", next_direction, next);

    (next, next_direction)
}

fn get_color(map: &HashMap<Point, i32>, location: &Point) -> i32 {
    match map.get(location) {
        Some(color) => *color,
        None => 0
    }
}

fn set_color(map: &mut HashMap<Point, i32>, location: Point, color: i32) {
    map.insert(location, color);
}

