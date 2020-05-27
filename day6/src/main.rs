use std::fs;
use std::time::Instant;

pub mod lib_bruteforce;
pub mod lib_hashmap;

fn main() {
    let input_str = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let input: Vec<&str> = input_str.lines().collect();

    let t0 = Instant::now();
    let mut system1 = lib_bruteforce::System::new();
    system1.parse_orbits(&input);
    println!("Old way");
    println!("Answer: {}", &system1.total_orbits());
    println!("Duration: {}ms", t0.elapsed().as_millis());

    println!();

    let t0 = Instant::now();
    let mut system2 = lib_hashmap::System::new();
    system2.parse_orbits(&input);
    println!("New way");
    println!("Answer: {}", &system2.total_orbits());
    println!("Duration: {}ms", t0.elapsed().as_millis());
}
