use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug)]
pub struct System {
    nodes: HashSet<String>,
    node_ids: HashMap<String, usize>,
    adj: HashMap<usize, Vec<usize>>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct SearchState {
    id: usize,
    length: usize,
}

impl System {
    pub fn new() -> Self {
        System {
            nodes: HashSet::new(),
            node_ids: HashMap::new(),
            adj: HashMap::new(),
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

    pub fn total_orbits(&self) -> usize {
        self.nodes
            .iter()
            .map(|o| self.depth(o))
            .filter_map(Result::ok)
            .sum()
    }

    pub fn depth(&self, o: &str) -> Result<usize, &'static str> {
        match self.shortest_path(o, "COM", true) {
            Some(d) => Ok(d),
            None => Err("Unable to find path"),
        }
    }

    pub fn minimun_orbital_transfers(
        &self,
        source: &str,
        target: &str,
    ) -> Result<usize, &'static str> {
        // Calculating orbital transfers so decrease by 2 since the current
        // orbits are taken into account when calculating paths
        match self.shortest_path(source, target, false) {
            Some(d) => Ok(d - 2),
            None => Err("Unable to find path"),
        }
    }

    fn shortest_path(&self, source: &str, target: &str, use_shortcut: bool) -> Option<usize> {
        if !self.node_ids.contains_key(source) || !self.node_ids.contains_key(target) {
            return None;
        }

        let s_id = *self.node_ids.get(source).unwrap();
        let t_id = *self.node_ids.get(target).unwrap();

        let mut dist: Vec<usize> = (0..self.node_ids.len()).map(|_| usize::MAX).collect();

        let mut heap = BinaryHeap::new();

        dist[s_id] = 0;

        heap.push(SearchState {
            id: s_id,
            length: 0,
        });

        while let Some(SearchState { id, length }) = heap.pop() {
            if id == t_id {
                return Some(length);
            }

            if length > dist[id] {
                continue;
            }

            // TODO: DRY
            if use_shortcut {
                let next = SearchState {
                    length: length + 1,
                    id: self.adj[&id][0],
                };

                if next.length < dist[next.id] {
                    heap.push(next);
                    dist[next.id] = next.length;
                }

                continue;
            }

            // TODO: DRY
            for n in &self.adj[&id] {
                let next = SearchState {
                    length: length + 1,
                    id: *n,
                };

                if next.length < dist[next.id] {
                    heap.push(next);
                    dist[next.id] = next.length;
                }
            }
        }

        None
    }

    fn add_orbiter(&mut self, o1: &str, o2: &str) {
        if self.nodes.insert(o1.to_string()) {
            // Assign an ID
            self.node_ids.insert(o1.to_string(), self.node_ids.len());
        }

        if self.nodes.insert(o2.to_string()) {
            // Assign an ID
            self.node_ids.insert(o2.to_string(), self.node_ids.len());
        }

        self.adj
            .entry(*self.node_ids.get(o1).unwrap())
            .or_insert_with(Vec::new)
            .push(*self.node_ids.get(o2).unwrap());

        self.adj
            .entry(*self.node_ids.get(o2).unwrap())
            .or_insert_with(Vec::new)
            .push(*self.node_ids.get(o1).unwrap());
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &SearchState) -> Ordering {
        other
            .length
            .cmp(&self.length)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &SearchState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn part_1_example() {
    let ex = [
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ];

    let mut system = System::new();

    system.parse_orbits(&ex);

    assert_eq!(system.depth("D").unwrap(), 3);
    assert_eq!(system.depth("L").unwrap(), 7);
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

    assert_eq!(system.minimun_orbital_transfers("YOU", "SAN").unwrap(), 4);
}
