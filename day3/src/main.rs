extern crate regex;

use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::cmp::Eq;
use std::fs;
use std::ops::Add;

#[derive(Debug, Copy, Clone, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn distance(self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Copy, Clone)]
struct Segment {
    p1: Point,
    p2: Point,
}

impl Segment {
    fn order_points(&self) -> Segment {
        if self.p1.x > self.p2.x || self.p1.y > self.p2.y {
            return Segment {
                p1: self.p2,
                p2: self.p1,
            };
        }

        Segment {
            p1: self.p1,
            p2: self.p2,
        }
    }

    fn contains_point(&self, p: Point) -> bool {
        let copy = self.order_points();

        if p.x == copy.p1.x || p.x == copy.p2.x {
            // X-axis matches, check Y
            if copy.p1.y <= p.y && p.y <= copy.p2.y {
                return true;
            }
        } else if p.y == copy.p1.y || p.y == copy.p2.y {
            // Y-axis matches, check X
            if copy.p1.x <= p.x && p.x <= copy.p2.x {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Clone)]
struct Path {
    segments: Vec<Segment>,
}

fn main() {
    let d = run("input.txt");

    if d > 0 {
        println!("Distance: {:?}", d);
    } else {
        println!("Unable to find crossing")
    }
}

fn run(input_file: &str) -> i32 {
    let input_str = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let paths: Vec<&str> = input_str.lines().collect();

    let origo = Point { x: 0, y: 0 };

    let path1 = cvt_points2path(cvt_str2points(origo, paths[0]));
    let path2 = cvt_points2path(cvt_str2points(origo, paths[1]));

    if let Some(p) = search(origo, &path1, &path2) {
        return origo.distance(p);
    }

    -1
}

fn search(origo: Point, path1: &Path, path2: &Path) -> Option<Point> {
    let mut closest: Option<Point> = None;
    let mut s_d = -1;

    for s1 in &path1.segments {
        for s2 in &path2.segments {
            if let Some(p) = intersection(s1, s2) {
                let d = p.distance(origo);
                if p != origo && s_d < 0 || d < s_d {
                    s_d = d;
                    closest = Some(p);
                }
            }
        }
    }

    closest
}

fn intersection(s1: &Segment, s2: &Segment) -> Option<Point> {
    let s1_x1 = min(s1.p1.x, s1.p2.x);
    let s2_x1 = min(s2.p1.x, s2.p2.x);

    let s1_y1 = min(s1.p1.y, s1.p2.y);
    let s2_y1 = min(s2.p1.y, s2.p2.y);

    let bot = min(s1_y1, s2_y1);
    let left = min(s1_x1, s2_x1);

    let bot_left = Point { x: left, y: bot };

    let diff = Point {
        x: max(s1_x1 - left, s2_x1 - left),
        y: max(s1_y1 - bot, s2_y1 - bot),
    };

    let p = bot_left + diff;

    if s1.contains_point(p) && s2.contains_point(p) {
        return Some(p);
    }

    None
}

fn cvt_str2points(origo: Point, path_str: &str) -> Vec<Point> {
    let path: Vec<&str> = path_str.split(',').collect();
    let re = Regex::new(r"^(?P<dir>\w{1})(?P<steps>\d+)$").unwrap();

    let mut points: Vec<Point> = Vec::with_capacity(path.len() as usize);

    points.push(origo);

    for i in 0..path.len() {
        assert!(re.is_match(path[i]));

        let caps = re.captures(path[i]).unwrap();

        let dir = caps.name("dir").map_or("", |m| m.as_str());

        let steps: i32 = caps
            .name("steps")
            .map_or("", |m| m.as_str())
            .parse()
            .unwrap();

        let diff: Point;

        match dir {
            "R" => diff = Point { x: steps, y: 0 },
            "L" => diff = Point { x: -steps, y: 0 },
            "U" => diff = Point { x: 0, y: steps },
            "D" => diff = Point { x: 0, y: -steps },
            x => panic!("unexpected direction: {:?}", x),
        }

        points.push(points[i] + diff);
    }

    points
}

fn cvt_points2path(points: Vec<Point>) -> Path {
    let mut segments: Vec<Segment> = Vec::with_capacity(points.len() - 1 as usize);
    for i in 1..points.len() {
        segments.push(Segment {
            p1: points[i - 1],
            p2: points[i],
        });
    }
    Path { segments }
}


#[test]
fn test1_txt() {
    assert_eq!(run("test1.txt"), 159);
}

#[test]
fn test2_txt() {
    assert_eq!(run("test2.txt"), 135);
}

#[test]
fn test3_txt() {
    assert_eq!(run("test3.txt"), 10);
}
