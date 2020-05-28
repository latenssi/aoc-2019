#[derive(Debug)]
pub struct System {
    orbiters: Vec<Orbiter>,
}

#[derive(Debug)]
struct Orbiter {
    key: String,
    orbiting_key: String,
}

impl System {
    pub fn new() -> Self {
        System { orbiters: vec![] }
    }

    fn add_orbiter(&mut self, key: &str, orbiting_key: &str) {
        let new_orbiter = Orbiter {
            key: key.to_string(),
            orbiting_key: orbiting_key.to_string(),
        };
        self.orbiters.push(new_orbiter);
    }

    pub fn parse_orbits(&mut self, input: &[&str]) {
        for line in input {
            let mut s = line.split(')');

            let p1 = s.next().unwrap();
            let p2 = s.next().unwrap();

            self.add_orbiter(p2, p1);
        }
    }

    pub fn depth(&self, key: String) -> i32 {
        let mut depth = 0;
        let mut next = key;
        while let Some(orbiter) = self.orbiters.iter().find(|o| o.key == next) {
            depth += 1;
            next = orbiter.orbiting_key.to_owned();
        }
        depth
    }

    pub fn total_orbits(&self) -> i32 {
        self.orbiters
            .iter()
            .map(|o| self.depth(o.key.to_string()))
            .sum()
    }
}

#[test]
fn part_1_example() {
    let ex = [
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ];

    let mut system = System { orbiters: vec![] };

    system.parse_orbits(&ex);

    assert_eq!(system.depth("D".to_string()), 3);
    assert_eq!(system.depth("L".to_string()), 7);
    assert_eq!(system.total_orbits(), 42);
}
