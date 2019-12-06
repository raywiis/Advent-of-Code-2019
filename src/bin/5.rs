fn main() {
    // let inputs: Vec<i32> = vec!(1);
    let inputs: Vec<i32> = vec!(5);
    let mut outputs: Vec<i32> = Vec::new();
    // let mut memory: Vec<i32> = vec!(1002,4,3,4,33);

    let mut memory: Vec<i32> = vec!(3,225,1,225,6,6,1100,1,238,225,104,0,1102,68,5,225,1101,71,12,225,1,117,166,224,1001,224,-100,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1001,66,36,224,101,-87,224,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1101,26,51,225,1102,11,61,224,1001,224,-671,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,1101,59,77,224,101,-136,224,224,4,224,1002,223,8,223,1001,224,1,224,1,223,224,223,1101,11,36,225,1102,31,16,225,102,24,217,224,1001,224,-1656,224,4,224,102,8,223,223,1001,224,1,224,1,224,223,223,101,60,169,224,1001,224,-147,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1102,38,69,225,1101,87,42,225,2,17,14,224,101,-355,224,224,4,224,102,8,223,223,1001,224,2,224,1,224,223,223,1002,113,89,224,101,-979,224,224,4,224,1002,223,8,223,1001,224,7,224,1,224,223,223,1102,69,59,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,7,677,677,224,1002,223,2,223,1006,224,329,1001,223,1,223,1007,226,226,224,1002,223,2,223,1006,224,344,1001,223,1,223,1108,226,677,224,102,2,223,223,1005,224,359,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,374,101,1,223,223,1107,677,226,224,1002,223,2,223,1006,224,389,101,1,223,223,7,226,677,224,1002,223,2,223,1005,224,404,101,1,223,223,1008,677,226,224,102,2,223,223,1005,224,419,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,434,101,1,223,223,107,226,226,224,1002,223,2,223,1005,224,449,1001,223,1,223,108,226,677,224,102,2,223,223,1005,224,464,101,1,223,223,1108,677,226,224,102,2,223,223,1005,224,479,101,1,223,223,1007,226,677,224,102,2,223,223,1006,224,494,101,1,223,223,107,677,677,224,102,2,223,223,1005,224,509,101,1,223,223,108,677,677,224,102,2,223,223,1006,224,524,1001,223,1,223,8,226,677,224,102,2,223,223,1005,224,539,101,1,223,223,107,677,226,224,102,2,223,223,1005,224,554,1001,223,1,223,8,226,226,224,102,2,223,223,1006,224,569,1001,223,1,223,7,677,226,224,1002,223,2,223,1005,224,584,1001,223,1,223,1108,226,226,224,102,2,223,223,1005,224,599,1001,223,1,223,1107,677,677,224,1002,223,2,223,1006,224,614,1001,223,1,223,1007,677,677,224,1002,223,2,223,1006,224,629,1001,223,1,223,108,226,226,224,102,2,223,223,1005,224,644,1001,223,1,223,8,677,226,224,1002,223,2,223,1005,224,659,1001,223,1,223,1008,677,677,224,1002,223,2,223,1006,224,674,1001,223,1,223,4,223,99,226);

    // let mut memory: Vec<i32> = vec!(3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99);

    // let inputs: Vec<i32> = vec!(8);
    // let mut memory = vec!(3,9,8,9,10,9,4,9,99,-1,8);

    // let inputs: Vec<i32> = vec!(8);
    // let mut memory = vec!(3,9,7,9,10,9,4,9,99,-1,8);

    // let inputs: Vec<i32> = vec!(9);
    // let mut memory = vec!(3,3,1108,-1,8,3,4,3,99);

    // let inputs: Vec<i32> = vec!(-3);
    // let mut memory = vec!(3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9);

    run_intcode(&mut memory, &inputs, &mut outputs);

    println!("{:?}", outputs);
}

enum ParamMode {
    Position,
    Immediate
}

fn unpack_opcode(opcode: i32) -> (i32, (ParamMode, ParamMode, ParamMode)) {
    let instruction = opcode % 100;
    let mut mode_digits = opcode / 100;
    let mut modes = (ParamMode::Position, ParamMode::Position, ParamMode::Position);

    if mode_digits > 0 {
        let digit = mode_digits % 10;
        mode_digits /= 10;
        modes.0 = match_mode(digit);
    }

    if mode_digits > 0 {
        let digit = mode_digits % 10;
        mode_digits /= 10;
        modes.1 = match_mode(digit);
    }

    if mode_digits > 0 {
        let digit = mode_digits % 10;
        modes.2 = match_mode(digit);
    }

    (instruction, modes)
}

fn match_mode(digit: i32) -> ParamMode {
    match digit {
        0 => ParamMode::Position,
        1 => ParamMode::Immediate,
        _ => panic!("Unsupported mode")
    }
}

fn fetch(memory: &Vec<i32>, mode: ParamMode, arg: i32) -> i32 {
    match mode {
        ParamMode::Position => memory[arg as usize],
        ParamMode::Immediate => arg
    }
}

fn run_intcode(memory: &mut Vec<i32>, inputs: &Vec<i32>, outputs: &mut Vec<i32>) {
    let mut instruction_ptr = 0;
    let mut input_ptr = 0;

    while memory[instruction_ptr] != 99 {
        let opcode = memory[instruction_ptr];
        let (instruction, modes) = unpack_opcode(opcode);

        match instruction {
            1 => {
                let a = fetch(memory, modes.0, memory[instruction_ptr + 1]);
                let b = fetch(memory, modes.1, memory[instruction_ptr + 2]);
                let target = memory[instruction_ptr + 3];

                std::mem::replace(&mut memory[target as usize], a + b);
                instruction_ptr += 4;
            },
            2 => {
                let a = fetch(memory, modes.0, memory[instruction_ptr + 1]);
                let b = fetch(memory, modes.1, memory[instruction_ptr + 2]);
                let target = memory[instruction_ptr + 3];

                std::mem::replace(&mut memory[target as usize], a * b);
                instruction_ptr += 4;
            },
            3 => {
                let input = inputs[input_ptr];
                input_ptr += 1;

                let destination_ptr = memory[instruction_ptr + 1];

                memory[destination_ptr as usize] = input;
                instruction_ptr += 2;
            },
            4 => {
                let output_ptr = memory[instruction_ptr + 1];
                let output = fetch(memory, modes.0, output_ptr);

                outputs.push(output);

                instruction_ptr += 2;
            },
            5 => {
                let test = fetch(memory, modes.0, memory[instruction_ptr + 1]);
                let target = fetch(memory, modes.1, memory[instruction_ptr + 2]);
                if test != 0 {
                    instruction_ptr = target as usize;
                } else {
                    instruction_ptr += 3;
                }
            },
            6 => {
                let test = fetch(memory, modes.0, memory[instruction_ptr + 1]);
                let target = fetch(memory, modes.1, memory[instruction_ptr + 2]);
                if test == 0 {
                    instruction_ptr = target as usize;
                } else {
                    instruction_ptr += 3;
                }
            },
            7 => {
                let p_1 = fetch(memory, modes.0, memory[instruction_ptr + 1]);
                let p_2 = fetch(memory, modes.1, memory[instruction_ptr + 2]);
                let target = memory[instruction_ptr + 3];

                if p_1 < p_2 {
                    memory[target as usize] = 1;
                } else {
                    memory[target as usize] = 0;
                }
                instruction_ptr += 4;
            },
            8 => {
                let p_1 = fetch(memory, modes.0, memory[instruction_ptr + 1]);
                let p_2 = fetch(memory, modes.1, memory[instruction_ptr + 2]);
                let target = memory[instruction_ptr + 3];

                if p_1 == p_2 {
                    memory[target as usize] = 1;
                } else {
                    memory[target as usize] = 0;
                }

                instruction_ptr += 4;
            },
            99 => {
                return;
            }
            _ => panic!("Unexpected instruction")
        };
    };
}

