use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct System {
    orbits: HashMap<String, String>,
    reverse: HashMap<String, Vec<String>>,
}

impl System {
    pub fn new() -> Self {
        System {
            orbits: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    pub fn parse_orbits(&mut self, input: &[&str]) {
        for line in input {
            let mut s = line.split(')');

            let p1 = s.next().unwrap();
            let p2 = s.next().unwrap();

            self.add_orbiter(p2, p1);
        }
    }

    pub fn total_orbits(&self) -> i32 {
        self.orbits.keys().map(|o| self.depth(o)).sum()
    }

    pub fn minimun_orbital_transfers(&self, source: &str, target: &str) -> u32 {
        // Calculating orbital transfers so decrease by 2 since the current
        // orbits are taken into account when calculating paths
        self.shortest_path(source, target) - 2
    }

    fn shortest_path(&self, source: &str, target: &str) -> u32 {
        let mut q: HashSet<&str> = HashSet::new();
        let mut dist: HashMap<&str, u32> = HashMap::new();

        // Init
        for n in self.orbits.keys() {
            dist.insert(n, u32::MAX);
            q.insert(n);
        }

        dist.insert(source, 0);

        // run
        while !q.is_empty() {
            // Get the next vertex in q with min dist
            let mut t: Option<&str> = None;
            let mut min_dist = u32::MAX;
            for (k, d) in dist
                .iter()
                .filter(|(k, d)| q.contains(*k) && **d < u32::MAX)
            {
                if *d < min_dist {
                    min_dist = *d;
                    t = Some(*k);
                }
            }

            if let Some(u) = t {
                if u == target {
                    // We are only interested in a shortest path between
                    // vertices 'source' and 'target'
                    break;
                }

                q.remove(u);

                for v in self.neighbours(u).iter().filter(|n| q.contains(*n)) {
                    let alt = *dist.get(u).unwrap() + 1; // All edges are equal
                    if alt < *dist.get(v).unwrap() {
                        dist.insert(*v, alt);
                    }
                }
            } else {
                // All remaning dists are u32::MAX
                break;
            }
        }

        *dist.get(target).unwrap()
    }

    fn add_orbiter(&mut self, orbiter: &str, orbiting: &str) {
        self.orbits
            .insert(orbiter.to_string(), orbiting.to_string());

        self.reverse
            .entry(orbiting.to_string())
            .or_insert(Vec::new())
            .push(orbiter.to_string());
    }

    fn depth(&self, orbiter: &str) -> i32 {
        let mut depth = 0;
        let mut next = orbiter;
        while let Some(primary_body) = self.orbits.get(next) {
            depth += 1;
            next = primary_body;
        }
        depth
    }

    fn neighbours(&self, orbiter: &str) -> Vec<&str> {
        let mut r: Vec<&str> = vec![];
        if let Some(o) = self.orbits.get(orbiter) {
            r.push(o);
        };
        if let Some(arr) = self.reverse.get(orbiter) {
            for e in arr {
                r.push(e);
            }
        };
        r
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

#[test]
fn part_2_example() {
    let ex = [
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
        "I)SAN",
    ];

    let mut system = System::new();

    system.parse_orbits(&ex);

    assert_eq!(system.minimun_orbital_transfers("YOU", "SAN"), 4);
}
