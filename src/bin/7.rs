use advent_of_code_2019::intcode::{execute, ExitCode};
use std::cmp::max;

fn main() {
    // let program = vec!(3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0);
    // let program = vec!(3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0);
    // let program = vec!(3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0);
    let program = vec!(3,8,1001,8,10,8,105,1,0,0,21,34,59,68,89,102,183,264,345,426,99999,3,9,102,5,9,9,1001,9,5,9,4,9,99,3,9,101,3,9,9,1002,9,5,9,101,5,9,9,1002,9,3,9,1001,9,5,9,4,9,99,3,9,101,5,9,9,4,9,99,3,9,102,4,9,9,101,3,9,9,102,5,9,9,101,4,9,9,4,9,99,3,9,1002,9,5,9,1001,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,99);
    // let program = vec!(3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5);

    let mut max_amp = 0;
    let seq_perms = permutations(vec!(), vec!(0, 1, 2, 3, 4));
    for seq in seq_perms {
        let amp_prom = amplify(&program, &seq).1;
        let amp = amp_prom.last().unwrap();
        max_amp = max(*amp, max_amp);
    };
    println!("[Day 7] Maximum amplification: {}", max_amp);

    let mut max_fdb = 0;
    let perms = permutations(vec!(), vec!(5, 6, 7, 8, 9));
    for perm in perms {
        let fdb = feedback(&program, perm.clone());
        max_fdb = max(fdb, max_fdb);
    }
    println!("[Day 7] Best feedback: {}", max_fdb);
}

fn permutations(prefix: Vec<i32>, rest: Vec<i32>) -> Vec<Vec<i32>> {
    if rest.is_empty() {
        return vec!(prefix);
    }

    let mut perms = vec!();
    for i in 0..rest.len() {
        let mut rest_copy = rest.clone();
        let add_elem = rest_copy.remove(i);
        let mut perm = prefix.clone();

        perm.push(add_elem);

        for tail in permutations(perm.clone(), rest_copy) {
            perms.push(tail);
        }
    }

    perms
}

fn feedback(program: &[i32], phase_seq: Vec<i32>) -> i32 {
    let outputs = vec!(0);
    let mut tuple = (ExitCode::NoInput, outputs.clone());
    while tuple.0 != ExitCode::Ok {
        for phase in &phase_seq {
            tuple = run_acs(program.to_vec(), *phase, &mut tuple.1.clone());
        }
        let mut orig = outputs.clone();
        orig.append(&mut tuple.1);
        tuple.1 = orig;
    }
    *tuple.1.last().unwrap()
}

fn amplify(program: &[i32], phase_seq: &[i32]) -> (ExitCode, Vec<i32>) {
    let mut tuple = (ExitCode::Ok, vec!(0));
    for phase in phase_seq {
        tuple = run_acs(program.to_vec(), *phase, &mut tuple.1.clone());
    }
    tuple
}

fn run_acs(mut memory: Vec<i32>, phase: i32, input: &mut Vec<i32>) -> (ExitCode, Vec<i32>) {
    let mut outputs: Vec<i32> = vec!();
    let mut inputs: Vec<i32> = vec!(phase);
    inputs.append(input);

    let exit = execute(&mut memory, &inputs, &mut outputs);

    (exit, outputs)
}
