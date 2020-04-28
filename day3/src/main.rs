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

    fn contains(&self, p: Point) -> bool {
        let s = self.order_points();

        if p.x == s.p1.x || p.x == s.p2.x {
            // X-axis matches, check Y
            if s.p1.y <= p.y && p.y <= s.p2.y {
                return true;
            }
        } else if p.y == s.p1.y || p.y == s.p2.y {
            // Y-axis matches, check X
            if s.p1.x <= p.x && p.x <= s.p2.x {
                return true;
            }
        }

        false
    }

    fn length(&self) -> i32 {
        let s = self.order_points();
        max(s.p2.x - s.p1.x, s.p2.y - s.p1.y)
    }
}

#[derive(Debug, Clone)]
struct Path {
    segments: Vec<Segment>,
}

impl Path {
    fn steps_to_reach(&self, point: Point) -> i32 {
        let mut steps = 0;
        for s in &self.segments {
            if s.contains(point) {
                let t_s = Segment {
                    p1: s.p1,
                    p2: point,
                };
                steps += t_s.length();
                break;
            } else {
                steps += s.length();
            }
        }
        steps
    }
}

fn main() {
    let d = part1("input.txt");

    if d > 0 {
        println!("Distance: {:?}", d);
    } else {
        println!("Unable to find crossing")
    }

    let steps = part2("input.txt");

    if steps > 0 {
        println!("Steps: {:?}", steps);
    } else {
        println!("Unable to find least steps")
    }
}

fn part1(input_file: &str) -> i32 {
    let input_str = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let paths: Vec<&str> = input_str.lines().collect();

    let origo = Point { x: 0, y: 0 };

    let p1 = cvt_points2path(cvt_str2points(origo, paths[0]));
    let p2 = cvt_points2path(cvt_str2points(origo, paths[1]));

    if let Some(d) = search_closest(origo, &p1, &p2) {
        return d;
    }

    -1
}

fn part2(input_file: &str) -> i32 {
    let input_str = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let paths: Vec<&str> = input_str.lines().collect();

    let origo = Point { x: 0, y: 0 };

    let p1 = cvt_points2path(cvt_str2points(origo, paths[0]));
    let p2 = cvt_points2path(cvt_str2points(origo, paths[1]));

    if let Some(s) = search_least_steps(&p1, &p2) {
        return s;
    }

    -1
}

fn search_closest(origo: Point, p1: &Path, p2: &Path) -> Option<i32> {
    let mut d: Vec<i32> = intersections(p1, p2)
        .into_iter()
        .map(|i| i.distance(origo))
        .collect();

    d.sort();

    match d.len() {
        len if len >= 2 => Some(d[1]),
        _ => None,
    }
}

fn search_least_steps(p1: &Path, p2: &Path) -> Option<i32> {
    let mut d: Vec<i32> = intersections(p1, p2)
        .into_iter()
        .map(|i| p1.steps_to_reach(i) + p2.steps_to_reach(i))
        .collect();

    d.sort();

    match d.len() {
        len if len >= 2 => Some(d[1]),
        _ => None,
    }
}

fn intersections(p1: &Path, p2: &Path) -> Vec<Point> {
    let mut v: Vec<Point> = Vec::new();
    for s1 in &p1.segments {
        for s2 in &p2.segments {
            if let Some(p) = intersection(s1, s2) {
                v.push(p)
            }
        }
    }
    v
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

    if s1.contains(p) && s2.contains(p) {
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
fn test_part1() {
    assert_eq!(part1("test1.txt"), 159);
    assert_eq!(part1("test2.txt"), 135);
    assert_eq!(part1("test3.txt"), 8);
}

#[test]
fn test_part2() {
    assert_eq!(part2("test1.txt"), 610);
    assert_eq!(part2("test2.txt"), 410);
    assert_eq!(part2("test3.txt"), 20);
}
