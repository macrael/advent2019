use std::convert::TryInto;

fn run_program(program: Vec<i32>) -> Vec<i32> {
    let mut prog = program.to_vec();

    let mut pc = 0;

    loop {
        let opcode = prog[pc];
        match opcode {
            1 => {
                // Addition
                let a_i: usize = prog[pc + 1].try_into().unwrap();
                let b_i: usize = prog[pc + 2].try_into().unwrap();
                let dest: usize = prog[pc + 3].try_into().unwrap();

                prog[dest] = prog[a_i] + prog[b_i];
            },
            2 => {
                // multplication
                let a_i: usize = prog[pc + 1].try_into().unwrap();
                let b_i: usize = prog[pc + 2].try_into().unwrap();
                let dest: usize = prog[pc + 3].try_into().unwrap();

                prog[dest] = prog[a_i] * prog[b_i];
            },
            99 => {
                // exit
                break;
            },
            _ => {
                // this is bad.panic!
                panic!();
            },
        }

        pc += 4

    }

    return prog;
}

const INPUT: [i32; 145] = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,6,19,1,19,6,23,2,23,6,27,2,6,27,31,2,13,31,35,1,9,35,39,2,10,39,43,1,6,43,47,1,13,47,51,2,6,51,55,2,55,6,59,1,59,5,63,2,9,63,67,1,5,67,71,2,10,71,75,1,6,75,79,1,79,5,83,2,83,10,87,1,9,87,91,1,5,91,95,1,95,6,99,2,10,99,103,1,5,103,107,1,107,6,111,1,5,111,115,2,115,6,119,1,119,6,123,1,123,10,127,1,127,13,131,1,131,2,135,1,135,5,0,99,2,14,0,0];

const GOAL: i32 = 19690720;

pub fn two_a() -> i32 {
    let mut input_prog = INPUT.to_vec();

    // first, fix some codes
    input_prog[1] = 12;
    input_prog[2] = 2;

    let output_prog = run_program(input_prog);

    return output_prog[0];
}

pub fn two_b() -> i32 {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut input_prog = INPUT.to_vec();
            input_prog[1] = noun;
             input_prog[2] = verb;

             let output_prog = run_program(input_prog);
             if output_prog[0] == GOAL {
                 return 100 * noun + verb;
             }
        } 
    }

    return -1
}

#[cfg(test)]
mod tests {
     use super::*;
    #[test]
    fn start() {
        assert_eq!(run_program(vec![1,0,0,0,99]), vec![2,0,0,0,99]);
        assert_eq!(run_program(vec![2,3,0,3,99]), vec![2,3,0,6,99]);
        assert_eq!(run_program(vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
        assert_eq!(run_program(vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
    }
}