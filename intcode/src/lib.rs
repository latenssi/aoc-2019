use bit_vec::BitVec;
use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Operation {
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

struct Instruction {
    operation: Operation,
    args: Vec<usize>,
}

struct Register {
    halt_flag: bool,
    jump_flag: bool,
    carry_flag: bool,
    sign_flag: bool,
    instruction_pointer: usize,
    input_stack: VecDeque<isize>,
}

pub struct Memory {
    raw: Vec<isize>,
}

type InstructionCall = fn(&mut Memory, &mut Register, &Instruction) -> Option<isize>;

struct InstructionSet {
    op_codes: HashMap<isize, (Operation, usize)>,
    instructions: HashMap<Operation, InstructionCall>,
}

pub struct Machine {
    instruction_set: InstructionSet,
    pub memory: Memory,
    register: Register,
}

impl Default for Register {
    fn default() -> Register {
        Register {
            halt_flag: false,
            jump_flag: false,
            carry_flag: false,
            sign_flag: false,
            instruction_pointer: 0,
            input_stack: VecDeque::<isize>::new(),
        }
    }
}

impl Register {
    fn halt_flag_set(&self) -> bool {
        self.halt_flag
    }

    fn set_halt_flag(&mut self) {
        self.halt_flag = true;
    }

    fn clear_halt_flag(&mut self) {
        self.halt_flag = false;
    }

    fn jump_flag_set(&self) -> bool {
        self.jump_flag
    }

    fn set_jump_flag(&mut self) {
        self.jump_flag = true;
    }

    fn clear_jump_flag(&mut self) {
        self.jump_flag = false;
    }

    fn set_carry_flag(&mut self) {
        self.carry_flag = true;
    }

    fn clear_carry_flag(&mut self) {
        self.carry_flag = false;
    }

    fn set_sign_flag(&mut self) {
        self.sign_flag = true;
    }

    fn clear_sign_flag(&mut self) {
        self.sign_flag = false;
    }

    fn incr_instruction_pointer(&mut self, incr: usize) {
        self.instruction_pointer += incr
    }

    fn set_instruction_pointer(&mut self, val: usize) {
        self.instruction_pointer = val
    }

    fn add_input(&mut self, val: isize) {
        self.input_stack.push_front(val)
    }

    fn get_input(&mut self) -> Option<isize> {
        self.input_stack.pop_back()
    }
}

impl Default for Memory {
    fn default() -> Memory {
        Memory {
            raw: Vec::<isize>::new(),
        }
    }
}

impl Memory {
    fn init(&mut self, program: &[isize]) {
        self.raw = program.to_vec();
    }

    pub fn get(&self, loc: usize) -> isize {
        if self.raw.len() <= loc {
            panic!("Memory overflow: {:?}", loc)
        }
        self.raw[loc]
    }

    fn get_val_loc(&self, pointer: usize, offset: usize, immediate: bool) -> usize {
        let location = pointer + offset;
        if immediate {
            location
        } else {
            self.get(location) as usize
        }
    }

    fn set(&mut self, loc: usize, val: isize) {
        self.raw[loc] = val
    }

    fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }
}

impl Default for InstructionSet {
    fn default() -> InstructionSet {
        InstructionSet {
            op_codes: HashMap::new(),
            instructions: HashMap::new(),
        }
    }
}

impl InstructionSet {
    fn insert(
        &mut self,
        code: isize,
        op: Operation,
        arg_len: usize,
        f: fn(&mut Memory, &mut Register, &Instruction) -> Option<isize>,
    ) {
        self.op_codes.insert(code, (op, arg_len));
        self.instructions.insert(op, f);
    }

    fn parse(&self, m: &Machine) -> Instruction {
        let code = m.memory.get(m.register.instruction_pointer);

        let op_code = code % 100;
        let modes = code / 100;

        let mut mode_v = BitVec::from_elem(3, false);

        mode_v.set(0, modes % 10 == 1);
        mode_v.set(1, (modes / 10) % 10 == 1);
        mode_v.set(2, (modes / 100) % 10 == 1);

        match self.op_codes.get(&(op_code)) {
            Some(x) => Instruction {
                operation: x.0,
                args: (0..x.1)
                    .map(|i| {
                        m.memory
                            .get_val_loc(m.register.instruction_pointer, i + 1, mode_v[i])
                    })
                    .collect(),
            },
            None => panic!("Unknown instruction"),
        }
    }

    fn execute(&self, m: &mut Memory, r: &mut Register, i: &Instruction) -> Option<isize> {
        match self.instructions.get(&i.operation) {
            Some(f) => f(m, r, i),
            None => panic!("Unknown operation"),
        }
    }
}

impl Default for Machine {
    fn default() -> Machine {
        let mut instruction_set = InstructionSet::default();

        instruction_set.insert(1, Operation::ADD, 3, |m, r, i| {
            let arg1 = m.get(i.args[0]);
            let arg2 = m.get(i.args[1]);

            let (res, overflow) = arg1.overflowing_add(arg2);
            if overflow {
                // 'res' is the wrapped around value
                println!("Wrapped: {}", res);
                r.sign_flag = res < 0;
                r.set_carry_flag();
            }
            m.set(i.args[2], res);
            None
        });

        instruction_set.insert(2, Operation::MUL, 3, |m, r, i| {
            let arg1 = m.get(i.args[0]);
            let arg2 = m.get(i.args[1]);

            let (res, overflow) = arg1.overflowing_mul(arg2);
            if overflow {
                // 'res' is the wrapped around value
                println!("Wrapped: {}", res);
                r.sign_flag = res < 0;
                r.set_carry_flag();
            }
            m.set(i.args[2], res);
            None
        });

        instruction_set.insert(3, Operation::IN, 1, |m, r, i| {
            m.set(i.args[0], r.get_input().expect("No input"));
            None
        });

        instruction_set.insert(4, Operation::OUT, 1, |m, r, i| {
            if r.carry_flag {}
            Some(m.get(i.args[0]))
        });

        instruction_set.insert(5, Operation::JIT, 2, |m, r, i| {
            if m.get(i.args[0]) != 0 {
                r.set_instruction_pointer(m.get(i.args[1]) as usize);
                r.set_jump_flag();
            }
            None
        });

        instruction_set.insert(6, Operation::JIF, 2, |m, r, i| {
            if m.get(i.args[0]) == 0 {
                r.set_instruction_pointer(m.get(i.args[1]) as usize);
                r.set_jump_flag();
            }
            None
        });

        instruction_set.insert(7, Operation::LT, 3, |m, _r, i| {
            m.set(i.args[2], (m.get(i.args[0]) < m.get(i.args[1])) as isize);
            None
        });

        instruction_set.insert(8, Operation::EQ, 3, |m, _r, i| {
            m.set(i.args[2], (m.get(i.args[0]) == m.get(i.args[1])) as isize);
            None
        });

        instruction_set.insert(99, Operation::HALT, 0, |_m, r, _i| {
            r.set_halt_flag();
            None
        });

        Machine {
            instruction_set,
            memory: Memory::default(),
            register: Register::default(),
        }
    }
}

impl Machine {
    pub fn init(&mut self, program: &[isize]) {
        self.memory.init(program);
        self.register = Register::default();
    }

    pub fn load_program(&mut self, path: &str) {
        let input_str = fs::read_to_string(path).expect("Something went wrong reading the file");

        let program: Vec<isize> = input_str
            .split(',')
            .map(|s| s.parse::<isize>().expect("Unable to parse integer"))
            .collect();

        self.init(&program);
    }

    pub fn input(&mut self, input: isize) {
        self.register.add_input(input)
    }

    pub fn run(&mut self) -> Option<isize> {
        if self.memory.is_empty() {
            panic!("No program loaded");
        }

        let mut final_output: Option<isize> = None;

        loop {
            let instruction = self.instruction_set.parse(self);

            if let Some(output) =
                self.instruction_set
                    .execute(&mut self.memory, &mut self.register, &instruction)
            {
                println!("{:?}", output);
                final_output = Some(output);
            }

            if self.register.halt_flag_set() {
                self.register.clear_halt_flag();
                break;
            }

            if self.register.jump_flag_set() {
                self.register.clear_jump_flag();
            } else {
                self.register
                    .incr_instruction_pointer(instruction.args.len() + 1);
            }
        }

        final_output
    }
}

#[test]
fn test_day2_puzzle1() {
    let mut machine = Machine::default();

    machine.load_program("test.txt");
    machine.run();

    assert_eq!(machine.memory.get(0), 4_462_686);
}

#[test]
fn test_basic_io() {
    let mut machine = Machine::default();
    machine.init(&[3, 0, 4, 0, 99]);
    machine.input(1337);
    let output = machine.run();

    assert_eq!(output, Some(1337));
}

#[test]
fn test_day5_comparison() {
    let mut m = Machine::default();

    let p1 = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let p2 = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    let p3 = [3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let p4 = [3, 3, 1107, -1, 8, 3, 4, 3, 99];

    // Equal to 8, position mode

    m.init(&p1);
    m.input(8);
    assert_eq!(m.run(), Some(1));

    m.init(&p1);
    m.input(1337);
    assert_eq!(m.run(), Some(0));

    // Less than 8, position  mode

    m.init(&p2);
    m.input(7);
    assert_eq!(m.run(), Some(1));

    m.init(&p2);
    m.input(9);
    assert_eq!(m.run(), Some(0));

    // Equal to 8, immediate mode

    m.init(&p3);
    m.input(8);
    assert_eq!(m.run(), Some(1));

    m.init(&p3);
    m.input(1337);
    assert_eq!(m.run(), Some(0));

    // Less than 8, immediate mode

    m.init(&p4);
    m.input(7);
    assert_eq!(m.run(), Some(1));

    m.init(&p4);
    m.input(9);
    assert_eq!(m.run(), Some(0));
}

#[test]
fn test_day5_jump() {
    let mut m = Machine::default();

    let p1 = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    let p2 = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

    // Position mode

    m.init(&p1);
    m.input(0);
    assert_eq!(m.run(), Some(0));

    m.init(&p1);
    m.input(1337);
    assert_eq!(m.run(), Some(1));

    // Immediate mode

    m.init(&p2);
    m.input(0);
    assert_eq!(m.run(), Some(0));

    m.init(&p2);
    m.input(1337);
    assert_eq!(m.run(), Some(1));
}

#[test]
fn test_day5_larger_example() {
    let mut m = Machine::default();

    let p = [
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];

    // The above example program uses an input instruction to ask for a single number. The program will then output 999 if the input value is below 8, output 1000 if the input value is equal to 8, or output 1001 if the input value is greater than 8

    // Below 8
    m.init(&p);
    m.input(7);
    assert_eq!(m.run(), Some(999));

    // Equal to 8
    m.init(&p);
    m.input(8);
    assert_eq!(m.run(), Some(1000));

    // Greater than 8
    m.init(&p);
    m.input(9);
    assert_eq!(m.run(), Some(1001));
}
