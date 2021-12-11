use regex::Regex;
use std::fs;

fn main() {
  let f = fs::read_to_string("d2.txt").expect("no file");
  let re = Regex::new(r"(\w+)\s+(\d+)\n").unwrap();
  let mut depth = 0;
  let mut distance = 0;
  for cap in re.captures_iter(&f) {
    let n = &cap[2].parse::<i32>().unwrap();
    match &cap[1] {
      "up" => depth -= n,
      "down" => depth += n,
      "forward" => distance += n,
      _ => panic!("wtf"),
    }
  }
  println!("part1: {}", depth * distance);

  depth = 0;
  distance = 0;
  let mut aim = 0;

  for cap in re.captures_iter(&f) {
    let n = &cap[2].parse::<i32>().unwrap();
    match &cap[1] {
      "up" => aim -= n,
      "down" => aim += n,
      "forward" => {
        distance += n;
        depth += n * aim;
      }
      _ => panic!("wtf"),
    }
  }
  println!("part2: {}", depth * distance);
}
