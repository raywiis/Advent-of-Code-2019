#[macro_use]
extern crate lazy_static;

pub mod intcode {

    use std::sync::Mutex;
    use std::collections::HashMap;

    pub fn execute(
        memory: &mut Vec<i64>,
        inputs: &[i64],
        outputs: &mut Vec<i64>
    ) -> ExitCode {
        run_intcode(memory, inputs, outputs)
    }

    #[derive(PartialEq, Debug)]
    pub enum ParamMode {
        Position,
        Immediate,
        Relative
    }

    #[derive(PartialEq, Debug)]
    pub enum ExitCode {
        Ok,
        NoInput
    }

    lazy_static!{
        static ref EX_MEM: Mutex<HashMap<i64, i64>> = {
            Mutex::new(HashMap::new())
        };
    }

    fn run_intcode(memory: &mut Vec<i64>, inputs: &[i64], outputs: &mut Vec<i64>) -> ExitCode {
        let mut instruction_ptr = 0;
        let mut input_ptr = 0;
        let mut rel_base = 0;

        while memory[instruction_ptr] != 99 {
            let opcode = memory[instruction_ptr];
            let (instruction, modes) = unpack_opcode(opcode);

            // println!("Opcode {}", opcode);

            match instruction {
                1 => {
                    let a = fetch(memory, modes.0, memory[instruction_ptr + 1], rel_base);
                    let b = fetch(memory, modes.1, memory[instruction_ptr + 2], rel_base);
                    let target = memory[instruction_ptr + 3];

                    put(memory,modes.2,instruction_ptr + 3,rel_base,a + b);
                    // std::mem::replace(&mut memory[target as usize], a + b);
                    instruction_ptr += 4;
                },
                2 => {
                    let a = fetch(memory, modes.0, memory[instruction_ptr + 1], rel_base);
                    let b = fetch(memory, modes.1, memory[instruction_ptr + 2], rel_base);
                    let target = memory[instruction_ptr + 3];

                    put(memory,modes.2,instruction_ptr + 3,rel_base,a * b);
                    // std::mem::replace(&mut memory[target as usize], a * b);
                    instruction_ptr += 4;
                },
                3 => {
                    if input_ptr == inputs.len() {
                        // TODO: Mark as incomplete
                        return ExitCode::NoInput;
                    }
                    let input = inputs[input_ptr];
                    input_ptr += 1;

                    // println!("{:?}",modes);
                    let backup_mode = match modes.0 {
                        ParamMode::Immediate => ParamMode::Immediate,
                        ParamMode::Position => ParamMode::Position,
                        ParamMode::Relative => ParamMode::Relative
                    };
                    let destination_ptr = memory[instruction_ptr + 1];
                    // let destination_ptr = fetch(memory, modes.0, memory[instruction_ptr + 1], rel_base);

                    put(memory, modes.0, instruction_ptr + 1, rel_base, input);
                    instruction_ptr += 2;
                },
                4 => {
                    let output_ptr = memory[instruction_ptr + 1];
                    let output = fetch(memory, modes.0, output_ptr, rel_base);

                    outputs.push(output);

                    instruction_ptr += 2;
                },
                5 => {
                    let test = fetch(memory, modes.0, memory[instruction_ptr + 1], rel_base);
                    let target = fetch(memory, modes.1, memory[instruction_ptr + 2], rel_base);
                    if test != 0 {
                        instruction_ptr = target as usize;
                    } else {
                        instruction_ptr += 3;
                    }
                },
                6 => {
                    let test = fetch(memory, modes.0, memory[instruction_ptr + 1], rel_base);
                    let target = fetch(memory, modes.1, memory[instruction_ptr + 2], rel_base);
                    if test == 0 {
                        instruction_ptr = target as usize;
                    } else {
                        instruction_ptr += 3;
                    }
                },
                7 => {
                    let p_1 = fetch(memory, modes.0, memory[instruction_ptr + 1], rel_base);
                    let p_2 = fetch(memory, modes.1, memory[instruction_ptr + 2], rel_base);
                    // let target = memory[instruction_ptr + 3];

                    if p_1 < p_2 {
                        put(memory,modes.2,instruction_ptr + 3,rel_base,1);
                        // memory[target as usize] = 1;
                    } else {
                        put(memory,modes.2,instruction_ptr + 3,rel_base,0);
                        // memory[target as usize] = 0;
                    }
                    instruction_ptr += 4;
                },
                8 => {
                    let p_1 = fetch(memory, modes.0, memory[instruction_ptr + 1], rel_base);
                    let p_2 = fetch(memory, modes.1, memory[instruction_ptr + 2], rel_base);
                    // let target = memory[instruction_ptr + 3];
                    // println!("{}, {}, {}", p_1, p_2, target as usize);

                    if p_1 == p_2 {
                        put(memory,modes.2,instruction_ptr + 3,rel_base,1);
                        // memory[target as usize] = 1;
                    } else {
                        put(memory,modes.2,instruction_ptr + 3,rel_base,0);
                        // memory[target as usize] = 0;
                    }

                    instruction_ptr += 4;
                },
                9 => {
                    let new_base = fetch(memory, modes.0, memory[instruction_ptr + 1], rel_base);
                    rel_base += new_base;
                    instruction_ptr += 2;
                }
                99 => {
                    return ExitCode::Ok;
                }
                _ => panic!("Unexpected instruction")
            };
        };
        ExitCode::Ok
    }

    fn unpack_opcode(opcode: i64) -> (i64, (ParamMode, ParamMode, ParamMode)) {
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

    fn match_mode(digit: i64) -> ParamMode {
        match digit {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("Unsupported mode")
        }
    }

    fn fetch(memory: &[i64], mode: ParamMode, arg: i64, rel_base: i64) -> i64 {
        if mode == ParamMode::Immediate {
            return arg
        }

        let position = match mode {
            ParamMode::Position => arg as usize,
            ParamMode::Relative => (rel_base + arg) as usize,
            _ => panic!("Big fail")
        };

        // println!("Fetch from {}", position);
        if position < memory.len() {
            memory[position]
        } else {
            let map = EX_MEM.lock().unwrap();
            let key = &(position as i64);
            if map.contains_key(key) {
                map.get(key).unwrap().to_owned()
            } else {
                0
            }
        }
    }

    fn put(memory: &mut [i64], mode: ParamMode, arg_ptr: usize, rel_base: i64, value: i64) {
        let arg = memory[arg_ptr];

        let position: usize = match mode {
            ParamMode::Position => arg as usize,
            ParamMode::Relative => (rel_base + arg) as usize,
            ParamMode::Immediate => panic!("Broken guarantee")
        };

        // println!("Put into {}", position);
        if position < memory.len() {
            memory[position] = value;
        } else {
            EX_MEM.lock().unwrap().insert(position as i64, value);
        }
    }

}
