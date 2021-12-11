use std::fs;

fn main() {
  let f = fs::read_to_string("d9.txt")
    .expect("no file")
    .trim()
    .to_string();
  // let f = "2199943210
  // 3987894921
  // 9856789892
  // 8767896789
  // 9899965678";

  let input: Vec<Vec<i32>> = f
    .split_whitespace()
    .map(|line| {
      line
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect()
    })
    .collect();
  println!("input: {:?}\n", input);

  let mut low_points: Vec<(usize, usize, i32)> = vec![];
  for (y, line) in input.iter().enumerate() {
    for (x, z) in line.iter().enumerate() {
      let ne = neigh(&input, y, x);
      if ne.iter().all(|n| n.2 > *z) {
        low_points.push((y, x, *z));
      }
    }
  }
  println!("low_points: {:?}\n", low_points);
  let part1: i32 = low_points.iter().map(|p| p.2 + 1).sum();
  println!("{}", part1);

  let mut part2 = low_points
    .iter()
    .map(|low_point| basin(&input, *low_point).len())
    .collect::<Vec<usize>>();
  part2.sort();
  part2.reverse();
  let part2: usize = part2.iter().take(3).fold(1, |acc, x| acc * x);
  println!("{:?}", part2);
}

fn neigh(input: &Vec<Vec<i32>>, y: usize, x: usize) -> Vec<(usize, usize, i32)> {
  let max_y = input.len();
  let max_x = input[0].len();
  let y = y as i32;
  let x = x as i32;
  let coords: Vec<(i32, i32)> = vec![(y - 1, x), (y, x - 1), (y + 1, x), (y, x + 1)];
  coords
    .into_iter()
    .filter(|(y, x)| {
      // println!("{} {}", x, y);
      *y >= 0 && *y < max_y as i32 && *x >= 0 && *x < max_x as i32
    })
    .map(|(y, x)| (y as usize, x as usize, input[y as usize][x as usize]))
    .collect()
}

fn basin(input: &Vec<Vec<i32>>, low_point: (usize, usize, i32)) -> Vec<(usize, usize, i32)> {
  let mut points: Vec<(usize, usize, i32)> = vec![];
  fn extend(
    input: &Vec<Vec<i32>>,
    points: &mut Vec<(usize, usize, i32)>,
    point: (usize, usize, i32),
  ) {
    points.push(point);
    let neighs = neigh(input, point.0, point.1);
    for n in neighs {
      if n.2 < 9 && !points.contains(&n) {
        extend(input, points, n);
      }
    }
  }
  extend(input, &mut points, low_point);
  points
}
