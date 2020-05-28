use std::fs;
use std::time::Instant;

pub mod lib_binaryheap;
pub mod lib_bruteforce;
pub mod lib_hashmap;

fn main() {
    let input_str = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let input: Vec<&str> = input_str.lines().collect();

    // Way 1

    // let t2 = Instant::now();
    // let mut system1 = lib_bruteforce::System::new();
    // system1.parse_orbits(&input);
    // println!("Way 1");
    // println!("Answer part 1: {}", &system1.total_orbits());
    // println!("Duration: {:?}", t2.elapsed());

    // println!();

    // Way 2

    // let mut system = lib_hashmap::System::new();
    // println!("Way 2");

    // let t1 = Instant::now();

    // let t2 = Instant::now();
    // system.parse_orbits(&input);
    // println!("Duration: {:?}", t2.elapsed());

    // let t2 = Instant::now();
    // println!("Answer part 1: {}", &system.total_orbits());
    // println!("Duration: {:?}", t2.elapsed());

    // let t2 = Instant::now();
    // println!(
    //     "Answer part 2: {:?}",
    //     &system.minimun_orbital_transfers("YOU", "SAN").unwrap()
    // );
    // println!("Duration: {:?}", t2.elapsed());

    // println!("Total duration: {:?}", t1.elapsed());

    // println!();

    // Way 3

    let mut system3 = lib_binaryheap::System::new();
    println!("Way 3");

    let t1 = Instant::now();

    let t2 = Instant::now();
    system3.parse_orbits(&input);
    println!("Graph build duration: {:?}", t2.elapsed());

    let t2 = Instant::now();
    println!(
        "Answer part 1: {}, duration {:?}",
        &system3.total_orbits(),
        t2.elapsed()
    );

    let t2 = Instant::now();
    println!(
        "Answer part 2: {:?}, duration {:?}",
        &system3.minimun_orbital_transfers("YOU", "SAN").unwrap(),
        t2.elapsed()
    );

    println!("Total duration: {:?}", t1.elapsed());
}
