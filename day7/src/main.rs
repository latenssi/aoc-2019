use intcode::Machine;
use itertools::Itertools;
use std::cmp;
use std::fs;

fn main() {
    let input_str = fs::read_to_string("input").expect("Something went wrong reading the file");

    let program: Vec<isize> = input_str
        .split(',')
        .map(|s| s.parse::<isize>().expect("Unable to parse integer"))
        .collect();

    let mut amplifiers: Vec<_> = (0..5).map(|_| Machine::default()).collect();

    let mut max_output = 0;
    let mut output = 0;

    for seq in (0..5).permutations(5) {
        println!("{:?}", seq);
        for (i, a) in amplifiers.iter_mut().enumerate() {
            a.init(&program);
            a.input(seq[i]);
            a.input(output);
            output = a.run().unwrap();
        }
        max_output = cmp::max(output, max_output);
    }

    println!("Part 1 answer: {}", max_output);
}

#[test]
fn test_example_1() {
    let p = [
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];

    let seq = [4, 3, 2, 1, 0];

    let mut amplifiers: Vec<_> = (0..5).map(|_| Machine::default()).collect();

    let mut output = Some(0);
    for (i, a) in amplifiers.iter_mut().enumerate() {
        a.init(&p);
        a.input(seq[i]);
        a.input(output.unwrap());
        output = a.run();
    }

    assert_eq!(output, Some(43210));
}

#[test]
fn test_example_2() {
    let p = [
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ];

    let seq = [0, 1, 2, 3, 4];

    let mut amplifiers: Vec<_> = (0..5).map(|_| Machine::default()).collect();

    let mut output = Some(0);
    for (i, a) in amplifiers.iter_mut().enumerate() {
        a.init(&p);
        a.input(seq[i]);
        a.input(output.unwrap());
        output = a.run();
    }

    assert_eq!(output, Some(54321));
}

#[test]
fn test_example_3() {
    let p = [
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];

    let seq = [1, 0, 4, 3, 2];

    let mut amplifiers: Vec<_> = (0..5).map(|_| Machine::default()).collect();

    let mut output = Some(0);
    for (i, a) in amplifiers.iter_mut().enumerate() {
        a.init(&p);
        a.input(seq[i]);
        a.input(output.unwrap());
        output = a.run();
    }

    assert_eq!(output, Some(65210));
}

// 8782503524126251271 too high
