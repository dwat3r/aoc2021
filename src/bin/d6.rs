use std::{collections::HashMap, fs};

fn main() {
  let f = fs::read_to_string("d6.txt")
    .expect("no file")
    .trim()
    .to_string();
  // let f = "3,4,3,1,2";
  let fish: Vec<i64> = f.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
  let mut i = 80;
  let mut part1: Vec<i64> = day(fish.clone());
  while i > 1 {
    part1 = day(part1);
    i -= 1;
  }
  println!("{}", part1.len());

  let mut i = 256;
  let mut m = to_hm(fish.clone());
  while i > 0 {
    m = day_better(m);
    i -= 1;
  }
  let part2: i64 = m.values().sum();
  println!("{}", part2);
}
// for part1
fn day(fish: Vec<i64>) -> Vec<i64> {
  let mut new_gen = Vec::new();
  for fish in fish {
    if fish == 0 {
      new_gen.push(6);
      new_gen.push(8);
    } else {
      new_gen.push(fish - 1);
    }
  }
  new_gen
}
// for part2
fn to_hm(fish: Vec<i64>) -> HashMap<i64, i64> {
  let mut m = HashMap::new();
  for fish in fish {
    *m.entry(fish).or_insert(0) += 1;
  }
  m
}
fn day_better(m: HashMap<i64, i64>) -> HashMap<i64, i64> {
  let mut m2 = HashMap::new();
  for (fish, count) in &m {
    if *fish != 0 {
      *m2.entry(*fish - 1).or_insert(0) += *count;
    }
  }
  let zeros = m.get(&0).get_or_insert(&0).clone();
  *m2.entry(6).or_insert(0) += zeros;
  *m2.entry(8).or_insert(0) += zeros;
  m2
}
