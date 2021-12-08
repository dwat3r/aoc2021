use std::{collections::HashMap, fs, iter};

use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let f = fs::read_to_string("d5.txt").expect("no file");
    let f = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";

    let re = Regex::new(r"\s*(\d+),(\d+)\s+\->\s+(\d+),(\d+)").unwrap();
    let input: Vec<(Point, Point)> = re
        .captures_iter(&f)
        .map(|cap| {
            let ps: Vec<i32> = cap
                .iter()
                .skip(1)
                .flatten()
                .map(|x| x.as_str().trim().parse::<i32>().unwrap())
                .collect();
            (Point { x: ps[0], y: ps[1] }, Point { x: ps[2], y: ps[3] })
        })
        .collect();
    // println!("{:?}", input);
    let points: Vec<&(Point, Point)> = input
        .iter()
        .filter(|(p1, p2)| p1.x == p2.x || p1.y == p2.y)
        .collect();
    println!("{:#?}", points);
    let points: Vec<i32> = points
        .iter()
        .flat_map(|(p1, p2)| {
            let ret = if p1.x == p2.x {
                iter::repeat(p1.x).zip(p1.y..=p2.y)
            } else {
                (p1.x..=p2.x).zip(iter::repeat(p1.y))
            };
            ret
        })
        .collect();
    // println!("{:?}", points);
    let overlaps = points.iter().fold(HashMap::new(), |mut m, x| {
        *m.entry(x).or_insert(0) += 1;
        m
    });

    println!("{:?}", overlaps);
    let overlaps = overlaps.values().filter(|&&v| v > 1).count();
    println!("{}", overlaps);
}
