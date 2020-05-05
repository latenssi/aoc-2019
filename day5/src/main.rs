use bit_vec::BitVec;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum OP {
    ADD,
    MUL,
    IN,
    OUT,
    HALT,
}

fn main() {
    let input_str = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let program: Vec<isize> = input_str
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect();

    let mut op_codes = HashMap::new();

    op_codes.insert(1, OP::ADD);
    op_codes.insert(2, OP::MUL);
    op_codes.insert(3, OP::IN);
    op_codes.insert(4, OP::OUT);
    op_codes.insert(99, OP::HALT);

    run(&program, &op_codes);
}

fn run(program: &Vec<isize>, op_codes: &HashMap<isize, OP>) -> Vec<isize> {
    let mut memory = program.clone();
    let mut pointer = 0usize;

    println!("{:?}", (pointer, &memory));

    loop {
        if pointer >= memory.len() {
            break;
        }

        let op_code = memory[pointer] % 100;
        let modes = memory[pointer] / 100;

        let mut mode_v = BitVec::from_elem(3, false);

        mode_v.set(0, modes % 10 == 1);
        mode_v.set(1, (modes / 10) % 10 == 1);
        mode_v.set(2, (modes / 100) % 10 == 1);

        let args: Vec<isize> = (0..=2)
            .map(|i| get_arg(&memory, pointer, i + 1, mode_v[i]))
            .collect();

        println!("{:?}", (pointer, op_code, &mode_v, &args, &memory));

        match op_codes.get(&(op_code)) {
            Some(OP::ADD) => {
                memory[args[2] as usize] = args[0] + args[1];
                pointer += 4;
            }
            Some(OP::MUL) => {
                memory[args[2] as usize] = args[0] * args[1];
                pointer += 4;
            }
            Some(OP::IN) => {
                memory[args[0] as usize] = get_input();
                pointer += 2;
            }
            Some(OP::OUT) => {
                println!("{:?}", memory[args[0] as usize]);
                pointer += 2;
            }
            Some(OP::HALT) => break,
            None => panic!("Unknown instruction"),
        }
    }

    memory
}

fn get_arg(mem: &Vec<isize>, pointer: usize, offset: usize, mode: bool) -> isize {
    let index = mem[pointer + offset] as usize;
    mem[index]
}

fn get_input() -> isize {
    1
}

#[test]
fn test_day2_puzzle1() {
    let input_str = fs::read_to_string("test.txt").expect("Something went wrong reading the file");

    let mut program: Vec<isize> = input_str
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect();

    program[1] = 12;
    program[2] = 2;

    let mut op_codes = HashMap::new();

    op_codes.insert(1, OP::ADD);
    op_codes.insert(2, OP::MUL);
    op_codes.insert(99, OP::HALT);

    assert_eq!(run(&program, &op_codes)[0], 4_462_686);
}

#[test]
fn test_basic_io() {
    let input_str = fs::read_to_string("test2.txt").expect("Something went wrong reading the file");

    let program: Vec<isize> = input_str
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect();

    let mut op_codes = HashMap::new();

    op_codes.insert(3, OP::IN);
    op_codes.insert(4, OP::OUT);
    op_codes.insert(99, OP::HALT);

    run(&program, &op_codes);
}
