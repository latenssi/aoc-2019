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
    input_stack: Vec<isize>,
    jump_flag: bool,
}

impl Default for Machine {
    fn default() -> Machine {
        let mut op_codes = HashMap::new();

        op_codes.insert(1, (OPERATION::ADD, 3));
        op_codes.insert(2, (OPERATION::MUL, 3));
        op_codes.insert(3, (OPERATION::IN, 1));
        op_codes.insert(4, (OPERATION::OUT, 1));
        op_codes.insert(5, (OPERATION::JIT, 2));
        op_codes.insert(6, (OPERATION::JIF, 2));
        op_codes.insert(7, (OPERATION::LT, 3));
        op_codes.insert(8, (OPERATION::EQ, 3));
        op_codes.insert(99, (OPERATION::HALT, 0));

        Machine {
            memory: Vec::<isize>::new(),
            op_codes,
            instruction_pointer: 0,
            input_stack: Vec::<isize>::new(),
            jump_flag: false
        }
    }
}

impl Machine {
    fn init(&mut self, program: &[isize]) {
        self.memory = program.to_vec();
        self.instruction_pointer = 0;
    }

    fn load_program(&mut self, path: &str) {
        let input_str = fs::read_to_string(path).expect("Something went wrong reading the file");

        let program: Vec<isize> = input_str
            .split(',')
            .map(|s| s.parse::<isize>().expect("Unable to parse integer"))
            .collect();

        self.init(&program);
    }

    fn jump_flag_set(&self) -> bool{
        self.jump_flag
    }

    fn set_jump_flag(&mut self) {
        self.jump_flag = true;
    }

    fn clear_jump_flag(&mut self) {
        self.jump_flag = false;
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

            let instruction = self.parse_instruction(self.instruction_pointer);

            if let Some(output) = self.run_instruction(&instruction) {
                println!("{:?}", output);
                final_output = Some(output);
            }

            match instruction.operation {
                OPERATION::HALT => break,
                _ => {
                    if self.jump_flag_set() {
                        self.clear_jump_flag();
                    }
                    else {
                        self.instruction_pointer += instruction.args.len() + 1;
                    }
                }
            }
        }

        final_output
    }

    fn parse_instruction(&mut self, offset: usize) -> Instruction {
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
            OPERATION::JIT => {
                if self.memory[instruction.args[0]] != 0 {
                    self.instruction_pointer = self.memory[instruction.args[1]] as usize;
                    self.set_jump_flag();
                }
                None
            }
            OPERATION::JIF => {
                if self.memory[instruction.args[0]] == 0 {
                    self.instruction_pointer = self.memory[instruction.args[1]] as usize;
                    self.set_jump_flag();
                }
                None
            }
            OPERATION::LT => {
                self.memory[instruction.args[2]] =
                    (self.memory[instruction.args[0]] < self.memory[instruction.args[1]]) as isize;
                None
            }
            OPERATION::EQ => {
                self.memory[instruction.args[2]] =
                    (self.memory[instruction.args[0]] == self.memory[instruction.args[1]]) as isize;
                None
            }
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
    machine.init(&[3,0,4,0,99]);
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

    assert_eq!(output, Some(4_283_952));
}


#[test]
fn test_day5_comparison() {
    let mut m = Machine::default();

    let p1 = [3,9,8,9,10,9,4,9,99,-1,8];
    let p2 = [3,9,7,9,10,9,4,9,99,-1,8];
    let p3 = [3,3,1108,-1,8,3,4,3,99];
    let p4 = [3,3,1107,-1,8,3,4,3,99];

    // Equal to 8, position mode

    m.init(&p1);
    m.input_stack.push(8);
    assert_eq!(m.run(), Some(1));

    m.init(&p1);
    m.input_stack.push(1337);
    assert_eq!(m.run(), Some(0));

    // Less than 8, position  mode

    m.init(&p2);
    m.input_stack.push(7);
    assert_eq!(m.run(), Some(1));

    m.init(&p2);
    m.input_stack.push(9);
    assert_eq!(m.run(), Some(0));

    // Equal to 8, immediate mode

    m.init(&p3);
    m.input_stack.push(8);
    assert_eq!(m.run(), Some(1));

    m.init(&p3);
    m.input_stack.push(1337);
    assert_eq!(m.run(), Some(0));

    // Less than 8, immediate mode

    m.init(&p4);
    m.input_stack.push(7);
    assert_eq!(m.run(), Some(1));

    m.init(&p4);
    m.input_stack.push(9);
    assert_eq!(m.run(), Some(0));
}


#[test]
fn test_day5_jump() {
    let mut m = Machine::default();

    let p1 = [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    let p2 = [3,3,1105,-1,9,1101,0,0,12,4,12,99,1];

    // Position mode

    m.init(&p1);
    m.input_stack.push(0);
    assert_eq!(m.run(), Some(0));

    m.init(&p1);
    m.input_stack.push(1337);
    assert_eq!(m.run(), Some(1));

    // Immediate mode

    m.init(&p2);
    m.input_stack.push(0);
    assert_eq!(m.run(), Some(0));

    m.init(&p2);
    m.input_stack.push(1337);
    assert_eq!(m.run(), Some(1));
}


#[test]
fn test_day5_larger_example() {
    let mut m = Machine::default();

    let p = [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
    1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
    999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

    // The above example program uses an input instruction to ask for a single number. The program will then output 999 if the input value is below 8, output 1000 if the input value is equal to 8, or output 1001 if the input value is greater than 8

    // Below 8
    m.init(&p);
    m.input_stack.push(7);
    assert_eq!(m.run(), Some(999));

    // Equal to 8
    m.init(&p);
    m.input_stack.push(8);
    assert_eq!(m.run(), Some(1000));

    // Greater than 8
    m.init(&p);
    m.input_stack.push(9);
    assert_eq!(m.run(), Some(1001));
}
