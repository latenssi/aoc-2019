use std::fs;
use std::time::Instant;

pub mod lib_binaryheap;
pub mod lib_bruteforce;
pub mod lib_hashmap;

fn main() {
    let input_str = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let input: Vec<&str> = input_str.lines().collect();

    // Way 1

    // let t0 = Instant::now();
    // let mut system1 = lib_bruteforce::System::new();
    // system1.parse_orbits(&input);
    // println!("Way 1");
    // println!("Answer part 1: {}", &system1.total_orbits());
    // println!("Duration: {}ms", t0.elapsed().as_millis());

    // println!();

    // Way 2

    let mut system = lib_hashmap::System::new();
    println!("Way 2");

    let t1 = Instant::now();

    let t0 = Instant::now();
    system.parse_orbits(&input);
    println!("Duration: {}ms", t0.elapsed().as_millis());

    let t0 = Instant::now();
    println!("Answer part 1: {}", &system.total_orbits());
    println!("Duration: {}ms", t0.elapsed().as_millis());

    let t0 = Instant::now();
    println!(
        "Answer part 2: {:?}",
        &system.minimun_orbital_transfers("YOU", "SAN").unwrap()
    );
    println!("Duration: {}ms", t0.elapsed().as_millis());

    println!("Total duration: {}ms", t1.elapsed().as_millis());

    println!();

    // Way 3

    let mut system3 = lib_binaryheap::System::new();
    println!("Way 3");

    let t1 = Instant::now();

    let t0 = Instant::now();
    system3.parse_orbits(&input);
    println!("Duration: {}ms", t0.elapsed().as_millis());

    let t0 = Instant::now();
    println!("Answer part 1: {}", &system3.total_orbits());
    println!("Duration: {}ms", t0.elapsed().as_millis());

    let t0 = Instant::now();
    println!(
        "Answer part 2: {:?}",
        &system3.minimun_orbital_transfers("YOU", "SAN").unwrap()
    );
    println!("Duration: {}ms", t0.elapsed().as_millis());

    println!("Total duration: {}ms", t1.elapsed().as_millis());
}
