use std::convert::TryInto;

// code, mode1, mode2, mode3
fn parse_code(code: i32) -> (i32, i32, i32, i32) {
    let mut parsed_code = code;

    let ten_thousands = parsed_code / 10_000;
    parsed_code = parsed_code - (10_000 * ten_thousands);

    let thousands = parsed_code / 1_000;
    parsed_code = parsed_code - (1_000 * thousands);

    let hundreads = parsed_code / 100;
    parsed_code = parsed_code - (100 * hundreads);

    return (parsed_code, hundreads, thousands, ten_thousands);
}

// returns (program_state, output)
fn run_program(program: Vec<i32>, input: i32) -> (Vec<i32>, Vec<i32>) {
    let mut prog = program.to_vec();

    let mut pc = 0;

    let mut output = vec![];

    loop {
        let (opcode, m1, m2, m3) = parse_code(prog[pc]);
        println!("Opcode: {}", opcode);
        // let opcode = prog[pc];
        match opcode {
            1 => {
                // Addition
                // a
                let a: i32;
                if m1 == 1 {
                    a = prog[pc + 1];
                } else if m1 == 0 {
                    let a_i: usize = prog[pc + 1].try_into().unwrap();
                    a = prog[a_i];
                } else {
                    println!("Unexpected! a{}", m1);
                    panic!();
                }

                //b
                let b: i32;
                if m2 == 1 {
                    b = prog[pc + 2];
                } else if m2 == 0 {
                    let b_i: usize = prog[pc + 2].try_into().unwrap();
                    b = prog[b_i];
                } else {
                    println!("Unexpected! a{}", m1);
                    panic!();
                }

                // dest
                // let dest: i32;
                // if m3 == 1 {
                //     dest = prog[pc + 3];
                // } else if m3 == 0 {
                //     let d_i: usize = prog[pc + 3].try_into().unwrap();
                //     dest = prog[d_i];
                // } else {
                //     println!("Unexpected! a{}", m1);
                //     panic!();
                // }

                let dest: usize = prog[pc + 3].try_into().unwrap();

                prog[dest] = a + b;

                pc += 4
            }
            2 => {
                // multplication
                // a
                let a: i32;
                if m1 == 1 {
                    a = prog[pc + 1];
                } else if m1 == 0 {
                    let a_i: usize = prog[pc + 1].try_into().unwrap();
                    a = prog[a_i];
                } else {
                    println!("Unexpected! a{}", m1);
                    panic!();
                }

                //b
                let b: i32;
                if m2 == 1 {
                    b = prog[pc + 2];
                } else if m2 == 0 {
                    let b_i: usize = prog[pc + 2].try_into().unwrap();
                    b = prog[b_i];
                } else {
                    println!("Unexpected! a{}", m1);
                    panic!();
                }

                let dest: usize = prog[pc + 3].try_into().unwrap();

                prog[dest] = a * b;

                pc += 4
            }
            3 => {
                // store input
                let dest: usize = prog[pc + 1].try_into().unwrap();

                prog[dest] = input;

                pc += 2
            }
            4 => {
                // send output
                // a
                let a: i32;
                if m1 == 1 {
                    a = prog[pc + 1];
                } else if m1 == 0 {
                    let a_i: usize = prog[pc + 1].try_into().unwrap();
                    a = prog[a_i];
                } else {
                    println!("Unexpected! a{}", m1);
                    panic!();
                }

                // let index: usize = prog[pc + 1].try_into().unwrap();

                output.push(a);

                pc += 2
            }
            99 => {
                // exit
                break;
            }
            _ => {
                // this is bad.panic!
                println!("unknown Op: {}", opcode);
                panic!();
            }
        }
    }

    return (prog, output);
}

const INPUT: [i32; 678] = [
    3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1101, 9, 90, 224, 1001, 224, -99, 224, 4, 224,
    102, 8, 223, 223, 1001, 224, 6, 224, 1, 223, 224, 223, 1102, 26, 62, 225, 1101, 11, 75, 225,
    1101, 90, 43, 225, 2, 70, 35, 224, 101, -1716, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 4,
    224, 224, 1, 223, 224, 223, 1101, 94, 66, 225, 1102, 65, 89, 225, 101, 53, 144, 224, 101, -134,
    224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 5, 224, 1, 224, 223, 223, 1102, 16, 32, 224,
    101, -512, 224, 224, 4, 224, 102, 8, 223, 223, 101, 5, 224, 224, 1, 224, 223, 223, 1001, 43,
    57, 224, 101, -147, 224, 224, 4, 224, 102, 8, 223, 223, 101, 4, 224, 224, 1, 223, 224, 223,
    1101, 36, 81, 225, 1002, 39, 9, 224, 1001, 224, -99, 224, 4, 224, 1002, 223, 8, 223, 101, 2,
    224, 224, 1, 223, 224, 223, 1, 213, 218, 224, 1001, 224, -98, 224, 4, 224, 102, 8, 223, 223,
    101, 2, 224, 224, 1, 224, 223, 223, 102, 21, 74, 224, 101, -1869, 224, 224, 4, 224, 102, 8,
    223, 223, 1001, 224, 7, 224, 1, 224, 223, 223, 1101, 25, 15, 225, 1101, 64, 73, 225, 4, 223,
    99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1,
    99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1,
    99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225,
    225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225,
    225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 1008, 226, 677, 224, 1002, 223, 2, 223,
    1005, 224, 329, 1001, 223, 1, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 344, 101,
    1, 223, 223, 108, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 359, 101, 1, 223, 223, 108, 226,
    226, 224, 1002, 223, 2, 223, 1005, 224, 374, 1001, 223, 1, 223, 7, 226, 226, 224, 1002, 223, 2,
    223, 1006, 224, 389, 1001, 223, 1, 223, 8, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 404,
    1001, 223, 1, 223, 107, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 419, 101, 1, 223, 223,
    1008, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 434, 101, 1, 223, 223, 1107, 226, 677, 224,
    102, 2, 223, 223, 1005, 224, 449, 1001, 223, 1, 223, 107, 226, 226, 224, 102, 2, 223, 223,
    1006, 224, 464, 101, 1, 223, 223, 107, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 479, 1001,
    223, 1, 223, 8, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 494, 1001, 223, 1, 223, 1108, 226,
    677, 224, 102, 2, 223, 223, 1006, 224, 509, 101, 1, 223, 223, 1107, 677, 226, 224, 1002, 223,
    2, 223, 1005, 224, 524, 101, 1, 223, 223, 1008, 226, 226, 224, 1002, 223, 2, 223, 1005, 224,
    539, 101, 1, 223, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 554, 101, 1, 223, 223,
    1107, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 569, 1001, 223, 1, 223, 8, 226, 226, 224,
    1002, 223, 2, 223, 1006, 224, 584, 101, 1, 223, 223, 1108, 677, 677, 224, 102, 2, 223, 223,
    1005, 224, 599, 101, 1, 223, 223, 108, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 614, 101,
    1, 223, 223, 1007, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 629, 1001, 223, 1, 223, 7, 677,
    226, 224, 1002, 223, 2, 223, 1005, 224, 644, 101, 1, 223, 223, 1007, 226, 677, 224, 102, 2,
    223, 223, 1005, 224, 659, 1001, 223, 1, 223, 1108, 677, 226, 224, 102, 2, 223, 223, 1006, 224,
    674, 101, 1, 223, 223, 4, 223, 99, 226,
];

const GOAL: i32 = 19690720;

pub fn five_a() -> i32 {
    let input_prog = INPUT.to_vec();

    let (_, mut output) = run_program(input_prog, 1);

    let diagnostic = output.remove(output.len() - 1);

    for err in output {
        if err != 0 {
            println!("Got an err: {}", err);
            panic!();
        }
    }

    return diagnostic;
}

// pub fn two_b() -> i32 {
//     for noun in 0..99 {
//         for verb in 0..99 {
//             let mut input_prog = INPUT.to_vec();
//             input_prog[1] = noun;
//             input_prog[2] = verb;

//             let output_prog = run_program(input_prog);
//             if output_prog[0] == GOAL {
//                 return 100 * noun + verb;
//             }
//         }
//     }

//     return -1;
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn start() {
        assert_eq!(
            run_program(vec![1, 0, 0, 0, 99], -1),
            (vec![2, 0, 0, 0, 99], [].to_vec())
        );
        assert_eq!(
            run_program(vec![2, 3, 0, 3, 99], -1),
            (vec![2, 3, 0, 6, 99], [].to_vec())
        );
        assert_eq!(
            run_program(vec![2, 4, 4, 5, 99, 0], -1),
            (vec![2, 4, 4, 5, 99, 9801], [].to_vec())
        );
        assert_eq!(
            run_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], -1),
            (vec![30, 1, 1, 4, 2, 5, 6, 0, 99], [].to_vec())
        );
    }

    #[test]
    fn parse_code_test() {
        assert_eq!(parse_code(1101), (1, 1, 1, 0));
        assert_eq!(parse_code(1102), (2, 1, 1, 0));
        assert_eq!(parse_code(10101), (1, 1, 0, 1));
    }

    #[test]
    fn new_comp() {
        assert_eq!(
            run_program(vec![3, 0, 4, 0, 99], 42),
            (vec![42, 0, 4, 0, 99], vec![42])
        );

        assert_eq!(
            run_program(vec![1101, 100, -1, 4, 0], 42),
            (vec![1101, 100, -1, 4, 99], vec![])
        );

        assert_eq!(
            run_program(vec![1002, 4, 3, 4, 33], 42),
            (vec![1002, 4, 3, 4, 99], vec![])
        );
    }
}
