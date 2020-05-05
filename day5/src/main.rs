use bit_vec::BitVec;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Copy, Clone)]
enum OPERATION {
    ADD,
    MUL,
    IN,
    OUT,
    JIT,
    JIF,
    LT,
    EQ,
    HALT,
}

#[derive(Debug)]
struct Instruction {
    operation: OPERATION,
    args: Vec<usize>,
}

#[derive(Debug)]
struct Machine {
    memory: Vec<isize>,
    op_codes: HashMap<isize, (OPERATION, usize)>,
    instruction_pointer: usize,
    instructions: Vec<Instruction>,
    input_stack: Vec<isize>,
}

impl Default for Machine {
    fn default() -> Machine {
        let mut op_codes = HashMap::new();

        op_codes.insert(1, (OPERATION::ADD, 3));
        op_codes.insert(2, (OPERATION::MUL, 3));
        op_codes.insert(3, (OPERATION::IN, 1));
        op_codes.insert(4, (OPERATION::OUT, 1));
        op_codes.insert(5, (OPERATION::JIT, 3));
        op_codes.insert(6, (OPERATION::JIF, 3));
        op_codes.insert(7, (OPERATION::LT, 3));
        op_codes.insert(8, (OPERATION::EQ, 3));
        op_codes.insert(99, (OPERATION::HALT, 0));

        Machine {
            memory: Vec::<isize>::new(),
            op_codes,
            instruction_pointer: 0,
            instructions: Vec::<Instruction>::new(),
            input_stack: Vec::<isize>::new(),
        }
    }
}

impl Machine {
    fn init(&mut self, program: &[isize]) {
        self.memory = program.to_vec();
        self.instruction_pointer = 0;
        self.instructions = Vec::<Instruction>::new();
    }

    fn load_program(&mut self, path: &str) {
        let input_str = fs::read_to_string(path).expect("Something went wrong reading the file");

        let program: Vec<isize> = input_str
            .split(',')
            .map(|s| s.parse::<isize>().expect("Unable to parse integer"))
            .collect();

        self.init(&program);
    }

    fn run(&mut self) -> Option<isize> {
        if self.memory.is_empty() {
            panic!("No program loaded");
        }

        let mut final_output: Option<isize> = None;

        loop {
            if self.instruction_pointer >= self.memory.len() {
                panic!("Program did not halt")
            }

            let instruction = self.parse_next_instruction(self.instruction_pointer);

            if let Some(output) = self.run_instruction(&instruction) {
                println!("{:?}", output);
                final_output = Some(output);
            }

            match instruction.operation {
                OPERATION::HALT => break,
                _ => {
                    self.instruction_pointer += instruction.args.len() + 1;
                    self.instructions.push(instruction);
                }
            }
        }

        final_output
    }

    fn parse_next_instruction(&mut self, offset: usize) -> Instruction {
        let op_code = self.memory[offset] % 100;
        let modes = self.memory[offset] / 100;

        let mut mode_v = BitVec::from_elem(3, false);

        mode_v.set(0, modes % 10 == 1);
        mode_v.set(1, (modes / 10) % 10 == 1);
        mode_v.set(2, (modes / 100) % 10 == 1);

        match self.op_codes.get(&(op_code)) {
            Some(x) => Instruction {
                operation: x.0,
                args: (0..x.1)
                    .map(|i| self.get_val_loc(i + 1, mode_v[i]))
                    .collect(),
            },
            None => panic!("Unknown instruction"),
        }
    }

    fn run_instruction(&mut self, instruction: &Instruction) -> Option<isize> {
        match instruction.operation {
            OPERATION::HALT => None,
            OPERATION::ADD => {
                self.memory[instruction.args[2]] =
                    self.memory[instruction.args[0]] + self.memory[instruction.args[1]];
                None
            }
            OPERATION::MUL => {
                self.memory[instruction.args[2]] =
                    self.memory[instruction.args[0]] * self.memory[instruction.args[1]];
                None
            }
            OPERATION::IN => {
                self.memory[instruction.args[0]] = self.input_stack.pop().expect("No input");
                None
            }
            OPERATION::OUT => Some(self.memory[instruction.args[0]]),
            OPERATION::JIT => panic!("Not implemented"),
            OPERATION::JIF => panic!("Not implemented"),
            OPERATION::LT => panic!("Not implemented"),
            OPERATION::EQ => panic!("Not implemented"),
        }
    }

    fn get_val_loc(&self, offset: usize, immediate: bool) -> usize {
        let location = self.instruction_pointer + offset;
        if immediate {
            location
        } else {
            self.memory[location] as usize
        }
    }
}

fn main() {
    let mut machine = Machine::default();

    // Part 1
    machine.load_program("input.txt");
    machine.input_stack.push(1);
    machine.run();

    // Part 2
    machine.load_program("input.txt");
    machine.input_stack.push(5);
    machine.run();
}

#[test]
fn test_day2_puzzle1() {
    let mut machine = Machine::default();
    machine.load_program("test.txt");
    machine.run();

    assert_eq!(machine.memory[0], 4_462_686);
}

#[test]
fn test_basic_io() {
    let mut machine = Machine::default();
    machine.input_stack.push(1337);
    machine.load_program("test2.txt");
    let output = machine.run();

    assert_eq!(output, Some(1337));
}

#[test]
fn test_day5_puzzle1() {
    let mut machine = Machine::default();
    machine.load_program("input.txt");
    machine.input_stack.push(1);
    let output = machine.run();

    assert_eq!(output, Some(9_938_601));
}

#[test]
fn test_day5_puzzle2() {
    let mut machine = Machine::default();
    machine.load_program("input.txt");
    machine.input_stack.push(5);
    let output = machine.run();

    assert_eq!(output, Some(9_938_601));
}
