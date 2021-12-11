use std::{collections::HashMap, fs, iter};

use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let f = fs::read_to_string("d5.txt").expect("no file");
  // let f = "0,9 -> 5,9
  //   8,0 -> 0,8
  //   9,4 -> 3,4
  //   2,2 -> 2,1
  //   7,0 -> 7,4
  //   6,4 -> 2,0
  //   0,9 -> 2,9
  //   3,4 -> 1,4
  //   0,0 -> 8,8
  //   5,5 -> 8,2";

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
  let part1 = calc_part1(&input);
  println!("{}", part1);
  let part2 = calc_part2(&input);
  println!("{}", part2);
}

fn calc_part1(input: &Vec<(Point, Point)>) -> usize {
  let points: Vec<&(Point, Point)> = input
    .iter()
    .filter(|(p1, p2)| p1.x == p2.x || p1.y == p2.y)
    .collect();
  // println!("{:#?}", points);
  let points: Vec<(i32, i32)> = points
    .iter()
    .flat_map(|(p1, p2)| {
      let ret: Vec<(i32, i32)> = if p1.x == p2.x {
        iter::repeat(p1.x)
          .zip(i32::min(p1.y, p2.y)..=i32::max(p1.y, p2.y))
          .collect()
      } else {
        (i32::min(p1.x, p2.x)..=i32::max(p1.x, p2.x))
          .zip(iter::repeat(p1.y))
          .collect()
      };
      ret
    })
    .collect();
  println!("{:?}", points);
  let overlaps = points.iter().fold(HashMap::new(), |mut m, x| {
    *m.entry(x).or_insert(0) += 1;
    m
  });

  println!("{:?}", overlaps);
  overlaps.values().filter(|&&v| v > 1).count()
}

fn calc_part2(input: &Vec<(Point, Point)>) -> usize {
  let points: Vec<(i32, i32)> = input
    .iter()
    .flat_map(|(p1, p2)| {
      let ret: Vec<(i32, i32)> = if p1.x == p2.x {
        iter::repeat(p1.x)
          .zip(i32::min(p1.y, p2.y)..=i32::max(p1.y, p2.y))
          .collect()
      } else if p1.y == p2.y {
        (i32::min(p1.x, p2.x)..=i32::max(p1.x, p2.x))
          .zip(iter::repeat(p1.y))
          .collect()
      } else {
        ((if p1.x < p2.x {
          (p1.x..=p2.x).collect::<Vec<i32>>()
        } else {
          (p2.x..=p1.x).rev().collect::<Vec<i32>>()
        })
        .iter()
        .zip(
          (if p1.y < p2.y {
            (p1.y..=p2.y).collect::<Vec<i32>>()
          } else {
            (p2.y..=p1.y).rev().collect::<Vec<i32>>()
          })
          .iter(),
        ))
        .map(|(x, y)| (*x, *y)) // :D
        .collect::<Vec<(i32, i32)>>()
      };
      ret
    })
    .collect();
  println!("{:?}", points);
  let overlaps = points.iter().fold(HashMap::new(), |mut m, x| {
    *m.entry(x).or_insert(0) += 1;
    m
  });

  println!("{:?}", overlaps);
  overlaps.values().filter(|&&v| v > 1).count()
}
