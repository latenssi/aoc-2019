use std::fs;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

fn main() {
    let input_str = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let paths: Vec<&str> = input_str.lines().collect();

    println!("{:?}", path_to_points(paths[0]));
}

fn path_to_points(path_str: &str) -> Vec<Point> {
    let path: Vec<&str> = path_str.split(",").collect();
    let mut points: Vec<Point> = Vec::with_capacity(path.len() as usize);
    for i in 0..path.len() {
        match String::from(path[i]).chars().nth(0).as_ref() {
            "R" => println!("Right"),
            "L" => println!("Left"),
            "U" => println!("Up"),
            "D" => println!("Down"),
            _ => println!("???"),
        }
        points.push(Point { x: 2, y: 1 })
    }
    points
}
