use std::fs;

fn main() {
    let input_str =
        fs::read_to_string("program.txt").expect("Something went wrong reading the file");

    let program: Vec<u64> = input_str
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    println!("Puzzle1 answer: {:?}", puzzle1(&program));
    println!("Puzzle2 answer: {:?}", puzzle2(&program));
}

fn puzzle1(program: &Vec<u64>) -> u64 {
    let memory = run(&program, 12, 2);
    memory[0]
}

fn puzzle2(program: &Vec<u64>) -> u64 {
    for noun in 0..100 {
        for verb in 0..100 {
            let memory = run(&program, noun, verb);
            if memory[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

fn run(orig_program: &Vec<u64>, noun: u64, verb: u64) -> Vec<u64> {
    let mut program = orig_program.clone();
    program[1] = noun;
    program[2] = verb;
    for i in (0..program.len()).step_by(4) {
        let opcode = program[i];
        let idx_in1 = program[i + 1] as usize;
        let idx_in2 = program[i + 2] as usize;
        let idx_out = program[i + 3] as usize;
        match opcode {
            1 => program[idx_out] = program[idx_in1] + program[idx_in2],
            2 => program[idx_out] = program[idx_in1] * program[idx_in2],
            99 => break,
            _ => println!("???"),
        }
    }
    program
}
