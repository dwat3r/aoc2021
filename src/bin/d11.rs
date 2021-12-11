use std::fs;

fn main() {
  let f = fs::read_to_string("d11.txt")
    .expect("no file")
    .trim()
    .to_string();
  //   let f = "5483143223
  // 2745854711
  // 5264556173
  // 6141336146
  // 6357385478
  // 4167524645
  // 2176841721
  // 6882881134
  // 4846848554
  // 5283751526";
  let mut input = get_input(&f);
  println!("{:?}\n", input);
  let part1 = step(&mut input, 100);
  println!("{:?}\n", part1);
  let mut input = get_input(&f);
  let mut count = 0;
  loop {
    step(&mut input, 1);
    count += 1;
    let in_sync = input.iter().flatten().all(|&e| e == 0);
    if in_sync {
      break;
    }
  }
  println!("{}", count);
}

fn get_input(f: &String) -> Vec<Vec<i32>> {
  f.split_whitespace()
    .map(|line| {
      line
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect()
    })
    .collect::<Vec<Vec<i32>>>()
}

fn neigh(input: &Vec<Vec<i32>>, y: usize, x: usize) -> Vec<(usize, usize)> {
  let max_y = input.len();
  let max_x = input[0].len();
  let coords = (usize::saturating_sub(y, 1)..=y + 1)
    .flat_map(|y| (usize::saturating_sub(x, 1)..=x + 1).map(move |x| (y, x)))
    .filter(|(y, x)| *y < max_y && *x < max_x)
    .collect::<Vec<(usize, usize)>>();
  coords
}

fn step(input: &mut Vec<Vec<i32>>, iterations: u32) -> u32 {
  let mut flashes = 0;
  let mut flashed: Vec<(usize, usize)> = vec![];
  fn flash(
    y: usize,
    x: usize,
    input: &mut Vec<Vec<i32>>,
    flashed: &mut Vec<(usize, usize)>,
    flashes: &mut u32,
  ) {
    if input[y][x] > 9 {
      input[y][x] = 0;
      flashed.push((y, x));
      *flashes += 1;
      let neighs = neigh(input, y, x);
      for (y, x) in neighs {
        if !flashed.contains(&(y, x)) {
          input[y][x] += 1;
          flash(y, x, input, flashed, flashes);
        }
      }
    }
  }

  for _ in 0..iterations {
    flashed.clear();
    for y in 0..input.len() {
      for x in 0..input[y].len() {
        input[y][x] += 1;
      }
    }
    for y in 0..input.len() {
      for x in 0..input[y].len() {
        flash(y, x, input, &mut flashed, &mut flashes);
      }
    }
  }
  flashes
}
