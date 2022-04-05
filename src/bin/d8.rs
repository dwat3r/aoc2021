use std::fs;

fn main() {
  let f = fs::read_to_string("d8.txt")
    .expect("no file")
    .trim()
    .to_string();

  //   let f = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
  // edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
  // fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
  // fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
  // aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
  // fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
  // dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
  // bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
  // egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
  // gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

  let input: Vec<(Vec<&str>, Vec<&str>)> = f
    .lines()
    .map(|line| {
      let line: Vec<&str> = line
        .split_whitespace()
        .filter(|word| !word.contains('|'))
        .collect();
      (
        line.clone().into_iter().take(10).collect(),
        line.clone().into_iter().skip(10).collect(),
      )
    })
    .collect();

  //   println!("{:?}", &input);

  let part1 = input
    .iter()
    .flat_map(|(_, outs)| {
      outs
        .iter()
        .filter(|w| [2, 3, 4, 7].iter().any(|&l| l == w.len() as i32))
    })
    .count();
  println!("{}", part1);
  let part2 = input.iter().map(decode_line).sum::<i32>();
  println!("{:?}", part2);
}

fn decode_line(line: &(Vec<&str>, Vec<&str>)) -> i32 {
  let (ins, outs) = line;
  let mut positions: Vec<Vec<char>> = vec![('a'..='g').collect(); 7];
  let get_digits = |len| -> Vec<Vec<char>> {
    ins
      .iter()
      .filter(|w| w.len() == len)
      .map(|w| w.chars().collect())
      .collect()
  };
  let diff = |v1: &Vec<char>, v2: &Vec<char>| -> Vec<char> {
    v1.iter().filter(|e| !v2.contains(e)).cloned().collect()
  };

  // let match_to = |digit|

  let one: Vec<char> = get_digits(2)[0].clone();
  positions[2] = one.clone();
  positions[5] = one.clone();
  let seven: Vec<char> = get_digits(3)[0].clone();
  positions[0] = diff(&seven, &one);
  let four: Vec<char> = get_digits(4)[0].clone();
  positions[1] = diff(&four, &one);
  positions[3] = positions[1].clone();

  positions[4] = diff(&positions[4], &vec![one, four, seven].concat());
  positions[6] = positions[4].clone();

  let zero = get_digits(6)
    .into_iter()
    .find(|digit| {
      let b1 = digit.contains(&positions[3][0]);
      let b2 = digit.contains(&positions[3][1]);
      (b1 && !b2) || (!b1 && b2)
    })
    .unwrap();
  positions[3] = diff(&positions[1], &zero);
  positions[1] = diff(&positions[1], &positions[3]);

  let two = get_digits(5)
    .into_iter()
    .find(|digit| {
      let b1 = digit.contains(&positions[2][0]);
      let b2 = digit.contains(&positions[2][1]);
      let b3 = digit.contains(&positions[4][0]);
      let b4 = digit.contains(&positions[4][1]);
      ((b1 && !b2) || (!b1 && b2)) && b3 && b4
    })
    .unwrap();
  positions[5] = diff(&positions[2], &two);
  positions[2] = diff(&positions[2], &positions[5]);

  let five = get_digits(5)
    .into_iter()
    .find(|digit| {
      let b1 = digit.contains(&positions[4][0]);
      let b2 = digit.contains(&positions[4][1]);
      let b3 = &vec![0, 1, 3, 5]
        .iter()
        .all(|&i| digit.contains(&positions[i][0]));
      ((b1 && !b2) || (!b1 && b2)) && *b3
    })
    .unwrap();
  positions[4] = diff(&positions[6], &five);
  positions[6] = diff(&positions[6], &positions[4]);
  let positions = positions.iter().flatten().collect::<Vec<&char>>();

  let numbers = vec![
    vec![0, 1, 2, 4, 5, 6],
    vec![2, 5],
    vec![0, 2, 3, 4, 6],
    vec![0, 2, 3, 5, 6],
    vec![1, 2, 3, 5],
    vec![0, 1, 3, 5, 6],
    vec![0, 1, 3, 4, 5, 6],
    vec![0, 2, 5],
    vec![0, 1, 2, 3, 4, 5, 6],
    vec![0, 1, 2, 3, 5, 6],
  ]
  .into_iter()
  .map(|poss| {
    poss
      .into_iter()
      .map(|i| *positions[i])
      .collect::<Vec<char>>()
  })
  .collect::<Vec<Vec<char>>>();
  outs
    .iter()
    .flat_map(|&digit| {
      numbers
        .iter()
        .enumerate()
        .map(|(ix, poss)| {
          (
            ix,
            diff(&to_chars(digit), poss).len(),
            diff(poss, &to_chars(digit)).len(),
          )
        })
        .filter(|(_ix, size, size2)| *size == 0 && *size2 == 0)
        .map(|triple| triple.0)
    })
    .zip([1000, 100, 10, 1])
    .map(|(x, n)| x * n)
    .sum::<usize>() as i32
  // println!("{:?}", positions);
}

fn to_chars(str: &str) -> Vec<char> {
  str.chars().collect::<Vec<_>>()
}
