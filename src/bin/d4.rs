use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Input {
  numbers: Vec<u32>,
  boards: Vec<Vec<Vec<(u32, bool)>>>,
}

fn main() {
  let f = fs::read_to_string("d4.txt").expect("no file");
  //   let f = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

  // 22 13 17 11  0
  //  8  2 23  4 24
  // 21  9 14 16  7
  //  6 10  3 18  5
  //  1 12 20 15 19

  //  3 15  0  2 22
  //  9 18 13 17  5
  // 19  8  7 25 23
  // 20 11 10 24  4
  // 14 21 16 12  6

  // 14 21 17 24  4
  // 10 16 15  9 19
  // 18  8 23 26 20
  // 22 11 13  6  5
  //  2  0 12  3  7";

  let mut input = parse(&f.to_string());
  // println!("{:#?}", input);
  let part1 = draw_numbers(&mut input);
  println!("{}", part1);
}
fn draw_numbers(input: &mut Input) -> u32 {
  let mut ret: u32 = 0;
  for number in &input.numbers {
    for board in &mut input.boards {
      for line in &mut board.into_iter() {
        for n in &mut line.into_iter() {
          if n.0 == *number {
            n.1 = true;
          }
        }
      }
    }
    match check_winner(input) {
      Some(board) => {
        ret = board
          .iter()
          .flatten()
          .filter(|n| !n.1)
          .map(|n| n.0)
          .sum::<u32>()
          * number;
        break;
      }
      None => {
        continue;
      }
    }
  }
  ret
}

fn check_winner(input: &Input) -> Option<&Vec<Vec<(u32, bool)>>> {
  input.boards.iter().find(|board| {
    let horiz = board.iter().any(|line| line.iter().all(|n| n.1));
    let mut vert = vec![true; board.len()];
    board.iter().enumerate().for_each(|(y, line)| {
      line.iter().for_each(|n| vert[y] = vert[y] && n.1);
    });
    let vert = vert.iter().any(|x| *x);
    horiz || vert
  })
}

fn parse(f: &String) -> Input {
  let numbers = f
    .split_whitespace()
    .take(1)
    .next()
    .unwrap()
    .split(',')
    .map(|x| x.trim().parse::<u32>().unwrap())
    .collect::<Vec<u32>>();

  let boards = f.trim()
    .split("\n\n")
    .skip(1)
    .map(|board| {
      board
        .split('\n')
        .map(|line| {
          line
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|x| (x.trim().parse::<u32>().unwrap(), false))
            .collect::<Vec<(u32, bool)>>()
        })
        .collect::<Vec<Vec<(u32, bool)>>>()
    })
    .collect::<Vec<Vec<Vec<(u32, bool)>>>>();

  Input { numbers, boards }
}
