use std::{cmp::Ordering, fs};

fn main() {
  let f = fs::read_to_string("d3.txt").expect("no file");
  let size = f.split_whitespace().next().unwrap().len();

  let mut zeros = vec![0; size];
  let mut ones = vec![0; size];
  for line in f.split_whitespace() {
    for (i, bit) in line.chars().enumerate() {
      if bit == '0' {
        zeros[i] += 1;
      } else {
        ones[i] += 1;
      }
    }
  }
  let (gamma, epsilon) =
    zeros
      .iter()
      .zip(ones.iter())
      .enumerate()
      .fold((0, 0), |(acc0, acc1), (i, (z, o))| {
        if z > o {
          (acc0 + i32::pow(2, (size - 1 - i).try_into().unwrap()), acc1)
        } else {
          (acc0, acc1 + i32::pow(2, (size - 1 - i).try_into().unwrap()))
        }
      });
  println!("part1: {}", gamma * epsilon);
  //     let f = "00100
  // 11110
  // 10110
  // 10111
  // 10101
  // 01111
  // 00111
  // 11100
  // 10000
  // 11001
  // 00010
  // 01010";
  //     let size = f.split_whitespace().next().unwrap().len();
  let size: i32 = size.try_into().unwrap();
  let numbers: Vec<i32> = f
    .split_whitespace()
    .map(|n| i32::from_str_radix(n, 2).unwrap())
    .collect();
  let ogr = count_bits(&numbers, size - 1, true);
  let csr = count_bits(&numbers, size - 1, false);
  println!("{}", ogr * csr);
}
fn get_bit(n: i32, ix: i32) -> i32 {
  (n >> ix) & 1
}
fn common_bits(input: &[i32], ix: i32) -> (i32, i32) {
  input
    .iter()
    .map(|&n| get_bit(n, ix))
    .fold((0, 0), |(acc0, acc1), b| {
      if b == 1 {
        (acc0, acc1 + 1)
      } else {
        (acc0 + 1, acc1)
      }
    })
}

fn count_bits(input: &[i32], ix: i32, most: bool) -> i32 {
  let (zeros, ones) = common_bits(input, ix);
  let p = match zeros.cmp(&ones) {
    Ordering::Greater => 0,
    Ordering::Equal => 1,
    Ordering::Less => 1,
  };
  let p = if most { p } else { 1 - p };
  let ret = input
    .iter()
    .cloned()
    .filter(|&n| get_bit(n, ix) == p)
    .collect::<Vec<i32>>();
  if ret.len() == 1 {
    ret[0]
  } else {
    count_bits(&ret, ix - 1, most)
  }
}
