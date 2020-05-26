use std::fs;

#[derive(Debug)]
struct System {
    orbiters: Vec<Orbiter>,
}

#[derive(Debug)]
struct Orbiter {
    key: String,
    orbits: String,
}

impl System {
    fn add_orbiter(&mut self, key: String, orbits: String) {
        self.orbiters.push(Orbiter { key, orbits });
    }

    fn parse_orbits(&mut self, input: &[String]) {
        for line in input {
            let mut s = line.split(')');

            let p1 = s.next().unwrap();
            let p2 = s.next().unwrap();

            self.add_orbiter(p2.to_string(), p1.to_string());
        }
    }

    fn path_len(&self, key: String) -> i32 {
        let mut len = 0;
        let mut next = key;
        while let Some(orbiter) = self.orbiters.iter().find(|o| o.key == next) {
            next = orbiter.orbits.to_owned();
            len += 1;
        }
        len
    }

    fn total_orbits(&self) -> i32 {
        self.orbiters
            .iter()
            .map(|o| self.path_len(o.key.to_string()))
            .sum()
    }
}

fn main() {
    let input_str = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let input: Vec<String> = input_str.lines().map(|l| (*l).to_string()).collect();

    let mut system = System { orbiters: vec![] };

    system.parse_orbits(&input);

    println!("{:?}", system.total_orbits())
}

#[test]
fn part_1_example() {
    let ex: Vec<String> = [
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ]
    .iter()
    .map(|l| (*l).to_string())
    .collect();

    let mut system = System { orbiters: vec![] };

    system.parse_orbits(&ex);

    assert_eq!(system.path_len("D".to_string()), 3);
    assert_eq!(system.path_len("L".to_string()), 7);
    assert_eq!(system.total_orbits(), 42);
}
