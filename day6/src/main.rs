// use std::convert::TryFrom;
use std::mem;
pub mod lib;

use lib::List;

// impl Node {
//     fn children<'a>(&self, nodes: &'a [Node]) -> Vec<&'a Node> {
//         nodes.iter().filter(|n| n.parent == self.key).collect::<Vec<&Node>>()
//     }
// }

// impl TryFrom<&&str> for Node {
//     type Error = ();

//     fn try_from(line: &&str) -> Result<Self, Self::Error> {
//         Node::try_from(line.split(')').collect::<Vec<&str>>())
//     }
// }

// impl TryFrom<Vec<&str>> for Node {
//     type Error = ();

//     fn try_from(split: Vec<&str>) -> Result<Self, Self::Error> {
//         if split.len() == 2 {
//             Ok(Node {
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
    // let l: List<char> = List::new();

    // println!("{:?}", 0x010);
}

// fn parse_nodes(input: &[&str]) -> Vec<Node> {
//     input
//         .iter()
//         .map(Node::try_from)
//         .filter_map(Result::ok)
//         .collect()
// }

// fn calculate_checksum(list: List) -> u32 {
//     0
// }

#[test]
fn part_1_example() {
    let ex = [
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ];

    let l: List<char> = List::new();

    // let list = List { root: "COM".to_string(), nodes: &parse_nodes(&ex) };

    // println!("{:?}", list);
    // assert_eq!(calculate_checksum(list), 42);
}
