use intcode::Machine;

fn main() {
    let mut machine = Machine::default();

    // Part 1
    machine.load_program("input.txt");
    machine.input(1);
    machine.run();

    // Part 2
    machine.load_program("input.txt");
    machine.input(5);
    machine.run();
}

#[test]
fn test_day5_puzzle1() {
    let mut machine = Machine::default();
    machine.load_program("input.txt");
    machine.input(1);
    let output = machine.run();

    assert_eq!(output, Some(9_938_601));
}

#[test]
fn test_day5_puzzle2() {
    let mut machine = Machine::default();
    machine.load_program("input.txt");
    machine.input(5);
    let output = machine.run();

    assert_eq!(output, Some(4_283_952));
}
