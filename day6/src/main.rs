// use std::collections::LinkedList;

// pub struct System {
//     root: Orbit,
// }

// type Orbit = Option<Box<Orbiter>>;

#[derive(Debug)]
struct Orbiter {
    key: char,
    parent: Option<Box<Orbiter>>,
}

impl PartialEq for Orbiter {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Orbiter {}

// impl Node {
//     fn children<'a>(&self, nodes: &'a [Node]) -> Vec<&'a Node> {
//         nodes
//             .iter()
//             .filter(|n| n.parent == self.elem)
//             .collect::<Vec<&Node>>()
//     }
// }

// impl TryFrom<&&str> for Orbiter {
//     type Error = ();

//     fn try_from(line: &&str) -> Result<Self, Self::Error> {
//         Orbiter::try_from(line.split(')').collect::<Vec<&str>>())
//     }
// }

// impl TryFrom<Vec<&str>> for Orbiter {
//     type Error = ();

//     fn try_from(split: Vec<&str>) -> Result<Self, Self::Error> {
//         if split.len() == 2 {
//             Ok(Orbiter {
//                 key: split[1].parse().unwrap(),
//                 parent: split[0].parse().unwrap(),
//             })
//         } else {
//             Err(())
//         }
//     }
// }

// #[derive(Debug)]
// struct Tree <'a>{
//     root: String,
//     nodes: &'a [Node]
// }

// impl Tree <'_> {
//     fn generate(&self) {}
// }

fn main() {
    let ex = [
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ];

    parse_orbits(&ex);
}

fn parse_orbits(input: &[&str]) {
    let orbiters: Vec<Box<Orbiter>> = vec![];

    for line in input {
        let mut s = line.split(')');

        let p = s.next().unwrap();
        let o = s.next().unwrap();

        let parent = orbiters.iter().find(|i| i.key == 'p');

        // let parent = Orbiter {
        //     key: 'p',
        //     parent: None,
        // };

        let orbiter = Orbiter { key: 'o', parent };

        println!("{:?}", (line, p, o));
    }

    // input
    //     .iter()
    //     .map(|orbit_str| {
    //         println!("{:?}", orbit_str);
    //     })
    //     // .filter_map(Result::ok)
    //     .collect();
}

// fn calculate_checksum(list: List) -> u32 {
//     0
// }

#[test]
fn part_1_example() {
    let ex = [
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ];

    parse_orbits(&ex);

    // let l: LinkedList<char> = LinkedList::new();

    // let list = List { root: "COM".to_string(), nodes: &parse_orbits(&ex) };

    // println!("{:?}", list);
    // assert_eq!(calculate_checksum(list), 42);
}
