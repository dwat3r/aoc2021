use std::collections::HashMap;
use std::fs;

fn main() {
    let f = fs::read_to_string("d10.txt")
        .expect("no file")
        .trim()
        .to_string();
    // let f = "[({(<(())[]>[[{[]{<()<>>
    // [(()[<>])]({[<{<<[]>>(
    // {([(<{}[<>[]}>{[]{[(<()>
    // (((({<>}<{<{<>}{[]{[]{}
    // [[<[([]))<([[{}[[()]]]
    // [{[{({}]{}}([{[{{{}}([]
    // {<[[]]>}<{[{[{[]{()[[[]
    // [<(<(<(<{}))><([]([]()
    // <{([([[(<>()){}]>(<<{{
    // <{([{{}}[<[[[<>{}]]]>[]]";

    let input = f
        .split_whitespace()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    // let part1 = find_corrupted(&"{([(<{}[<>[]}>{[]{[(<()>".chars().collect::<Vec<char>>());
    let part1: i32 = input
        .iter()
        .flat_map(|line| find_corrupted(&line))
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("wft"),
        })
        .sum();
    println!("{:?}", part1);
    let mut part2 = input
        .iter()
        .filter(|line| find_corrupted(&line).is_none())
        .map(|line| {
            let compl = complete_line(&line);
            calc_score(&compl)
        })
        .collect::<Vec<i64>>();

    part2.sort();
    let middle = part2.len() / 2;

    println!("{:?}", part2[middle]);
}

fn find_corrupted(input: &Vec<char>) -> Option<char> {
    let pairs = HashMap::from([('<', '>'), ('[', ']'), ('(', ')'), ('{', '}')]);
    let mut stack = vec![];
    let mut ret = None;
    for c in input {
        // println!("{:?}", stack);
        if pairs.keys().collect::<Vec<&char>>().contains(&&c) {
            stack.push(c)
        } else {
            let matching = stack.pop();
            match matching {
                None => ret = None,
                Some(open) => {
                    // println!("{} {} {:?}", c, open, pairs.get(&open));
                    if pairs.get(&open) == Some(&c) {
                        continue;
                    } else {
                        ret = Some(*c);
                    }
                }
            }
        }
    }
    ret
}

fn complete_line(input: &Vec<char>) -> Vec<char> {
    let pairs = HashMap::from([('<', '>'), ('[', ']'), ('(', ')'), ('{', '}')]);
    let mut stack: Vec<char> = vec![];
    for c in input {
        // println!("{:?}", stack);
        if pairs.keys().collect::<Vec<&char>>().contains(&&c) {
            stack.push(*c)
        } else {
            let matching = stack.pop();
            match matching {
                None => panic!("overclosing!"),
                Some(open) => {
                    // println!("{} {} {:?}", c, open, pairs.get(&open));
                    if pairs.get(&open) == Some(&c) {
                        continue;
                    } else {
                        panic!("corrupted line!");
                    }
                }
            }
        }
    }
    let mut ret = stack
        .into_iter()
        .map(|c| pairs.get(&c).unwrap().clone())
        .collect::<Vec<char>>();
    ret.reverse();
    ret
}

fn calc_score(compl: &Vec<char>) -> i64 {
    compl
        .iter()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("wtf"),
        })
        .fold(0, |sum, score| sum * 5 + score)
}
