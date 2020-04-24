extern crate regex;

use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

fn main() {
    let input_str = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let paths: Vec<&str> = input_str.lines().collect();

    println!("{:?}", cvt_str2points(paths[0]));
}

fn cvt_str2points(path_str: &str) -> Vec<Point> {
    let path: Vec<&str> = path_str.split(",").collect();
    let mut points: Vec<Point> = Vec::with_capacity(path.len() as usize);

    let re = Regex::new(r"^(?P<direction>\w{1})(?P<steps>\d+)$").unwrap();

    for i in 0..path.len() {
        assert!(re.is_match(path[i]));

        let caps = re.captures(path[i]).unwrap();
        let dir = caps.name("direction").map_or("", |m| m.as_str());
        let steps = caps.name("steps").map_or("", |m| m.as_str());

        println!("{:?}", (dir, steps));

        // let matches: Vec<_> = path[i].matches(re).into_iter().collect();
        // println!("{:?}", matches)
        // match String::from(path[i]).chars().nth(0).as_ref() {
        //    Some('R') => println!("Right {:?}", path[i]),
        //    Some('L') => println!("Left"),
        //    Some('U') => println!("Up"),
        //    Some('D') => println!("Down"),
        //     _ => println!("???"),
        // }
        // points.push(Point { x: 2, y: 1 })
    }
    points
}
