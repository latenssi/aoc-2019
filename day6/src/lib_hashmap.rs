use std::collections::HashMap;

#[derive(Debug)]
pub struct System {
    orbiters: HashMap<String, String>,
}

impl System {
    pub fn new() -> Self {
        System {
            orbiters: HashMap::new(),
        }
    }

    fn add_orbiter(&mut self, key: &str, orbiting_key: &str) {
        self.orbiters
            .insert(key.to_string(), orbiting_key.to_string());
    }

    pub fn parse_orbits(&mut self, input: &[&str]) {
        for line in input {
            let mut s = line.split(')');

            let p1 = s.next().unwrap();
            let p2 = s.next().unwrap();

            self.add_orbiter(p2, p1);
        }
    }

    pub fn depth(&self, key: &str) -> i32 {
        let mut len = 0;
        let mut next = key;
        while let Some(primary_body) = self.orbiters.get(next) {
            len += 1;
            next = primary_body;
        }
        len
    }

    pub fn total_orbits(&self) -> i32 {
        self.orbiters.keys().map(|o| self.depth(o)).sum()
    }
}

#[test]
fn part_1_example() {
    let ex = [
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ];

    let mut system = System::new();

    system.parse_orbits(&ex);

    assert_eq!(system.depth("D"), 3);
    assert_eq!(system.depth("L"), 7);
    assert_eq!(system.total_orbits(), 42);
}
