pub mod intcode {
    pub fn execute(
        memory: &mut Vec<i32>,
        inputs: &[i32],
        outputs: &mut Vec<i32>
    ) -> ExitCode {
        run_intcode(memory, inputs, outputs)
    }

    pub enum ParamMode {
        Position,
        Immediate
    }

    #[derive(PartialEq)]
    pub enum ExitCode {
        Ok,
        NoInput
    }

    fn run_intcode(memory: &mut Vec<i32>, inputs: &[i32], outputs: &mut Vec<i32>) -> ExitCode {
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
                    if input_ptr == inputs.len() {
                        // TODO: Mark as incomplete
                        return ExitCode::NoInput;
                    }
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
                    return ExitCode::Ok;
                }
                _ => panic!("Unexpected instruction")
            };
        };
        ExitCode::Ok
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

    fn fetch(memory: &[i32], mode: ParamMode, arg: i32) -> i32 {
        match mode {
            ParamMode::Position => memory[arg as usize],
            ParamMode::Immediate => arg
        }
    }

}
