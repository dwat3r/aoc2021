use std::fs;

fn main() {
  let f = fs::read_to_string("d7.txt")
    .expect("no file")
    .trim()
    .to_string();
  // let f = "16,1,2,0,4,2,7,1,2,14";
  let input: Vec<i32> = f.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
  println!("{:?}", input);
  let part1: i32 = calc_part1(&input);
  println!("{}", part1);
  let part2: i32 = calc_part2(&input);
  println!("{}", part2);
}

fn calc_part1(v: &Vec<i32>) -> i32 {
  let min = *v.iter().min().expect("");
  let max = *v.iter().max().expect("");
  let v2: Vec<i32> = (min..(max + 1))
    .map(|val| v.iter().map(|x| (x - val).abs()).sum())
    .collect();
  println!("{:?}", v2);
  *v2.iter().min().expect("")
}

fn calc_part2(v: &Vec<i32>) -> i32 {
  let min = *v.iter().min().expect("");
  let max = *v.iter().max().expect("");
  let v2: Vec<i32> = (min..(max + 1))
    .map(|val| {
      v.iter()
        .map(|x| {
          let dist = (x - val).abs();
          dist * (dist + 1) / 2
        })
        .sum()
    })
    .collect();
  println!("{:?}", v2);
  *v2.iter().min().expect("")
}
