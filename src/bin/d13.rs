use std::{collections::HashSet, fs};

use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Dir {
  X,
  Y,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Input {
  paper: Vec<(u32, u32)>,
  instructions: Vec<(u32, Dir)>,
}

fn main() {
  let f = fs::read_to_string("d13.txt").expect("no file");
  // let f = "6,10
  // 0,14
  // 9,10
  // 0,3
  // 10,4
  // 4,11
  // 6,0
  // 6,12
  // 4,1
  // 0,13
  // 10,12
  // 3,4
  // 3,0
  // 8,4
  // 1,10
  // 2,14
  // 8,10
  // 9,0

  // fold along y=7
  // fold along x=5";
  let input = parse(&f);
  // println!("{:?}\n", input.paper);
  draw(
    input
      .paper
      .clone()
      .into_iter()
      .collect::<HashSet<(u32, u32)>>(),
  );
  let part1 = fold(&input, true);
  println!("{:?}", part1.len());
  draw(part1);

  let part2 = fold(&input, false);

  draw(part2);
}
fn draw(paper: HashSet<(u32, u32)>) {
  let max_x = paper.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
  let max_y = paper.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
  for y in 0..=max_y {
    for x in 0..=max_x {
      if paper.contains(&(x, y)) {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!();
  }
  println!();
}
fn fold(input: &Input, one: bool) -> HashSet<(u32, u32)> {
  let mut folded: HashSet<(u32, u32)> = input.paper.clone().into_iter().collect();
  for (loc, dir) in &input.instructions {
    folded = folded
      .iter()
      .filter(|(x, y)| match dir {
        Dir::X => x != loc,
        Dir::Y => y != loc,
      })
      .map(|(x, y)| match dir {
        Dir::X => (if x > loc { 2 * loc - x } else { *x }, *y),
        Dir::Y => (*x, if y > loc { 2 * loc - y } else { *y }),
      })
      .collect::<HashSet<(u32, u32)>>();
    if one {
      break;
    }
  }
  folded
}

fn parse(f: &str) -> Input {
  let paper_re = Regex::new(r"(\d+),(\d+)").unwrap();
  let dir_re = Regex::new(r"fold along (\w)=(\d+)").unwrap();
  let paper = paper_re
    .captures_iter(f)
    .map(|cap| {
      cap
        .iter()
        .skip(1)
        .flatten()
        .map(|x| x.as_str().trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
    })
    .map(|v| (v[0], v[1]))
    .collect::<Vec<(u32, u32)>>();
  let instrs = dir_re
    .captures_iter(f)
    .map(|cap| {
      let dir = match &cap[1] {
        "x" => Dir::X,
        "y" => Dir::Y,
        _ => panic!("wrong input"),
      };
      let loc = cap[2].parse::<u32>().unwrap();
      (loc, dir)
    })
    .collect::<Vec<(u32, Dir)>>();
  Input {
    paper,
    instructions: instrs,
  }
}
