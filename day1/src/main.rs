use std::fs;

fn main() {
    let input_str =
        fs::read_to_string("module_masses.txt").expect("Something went wrong reading the file");

    let module_masses: Vec<f64> = input_str
        .lines()
        .map(|s| s.parse::<f64>().unwrap())
        .collect();

    println!("Puzzle1 answer: {:?}", puzzle1(&module_masses));
    println!("Puzzle2 answer: {:?}", puzzle2(&module_masses));
}

fn puzzle1(module_masses: &Vec<f64>) -> f64 {
    module_masses
        .iter()
        .fold(0.0, |sum, mass| sum + calc_fuel_mass_for_mass(*mass))
}

fn puzzle2(module_masses: &Vec<f64>) -> f64 {
    module_masses
        .iter()
        .fold(0.0, |sum, mass| sum + calc_total_fuel_mass_for_mass(*mass))
}

fn calc_fuel_mass_for_mass(mass: f64) -> f64 {
    let fuel_mass = (mass / 3.0).floor() - 2.0;
    if fuel_mass < 0.0 {
        return 0.0;
    } else {
        return fuel_mass;
    }
}

fn calc_total_fuel_mass_for_mass(mass: f64) -> f64 {
    let mut total_fuel_mass = calc_fuel_mass_for_mass(mass);
    let mut fuel_mass_for_fuel = total_fuel_mass;
    loop {
        fuel_mass_for_fuel = calc_fuel_mass_for_mass(fuel_mass_for_fuel);
        if fuel_mass_for_fuel > 0.0 {
            total_fuel_mass += fuel_mass_for_fuel;
        } else {
            break;
        }
    }
    total_fuel_mass
}
