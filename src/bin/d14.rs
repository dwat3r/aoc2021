use std::{collections::HashMap, iter::zip};

use itertools::Itertools;

type Pairs<'a> = HashMap<&'a str, char>;
#[derive(Debug)]
struct Input<'a> {
    formula: &'a str,
    pairs: Pairs<'a>,
}

fn main() {
    // let f = fs::read_to_string("d14.txt").expect("no file");
    let f = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    let input = get_input(f);
    println!("{:?}", input);
    let part1 = iterate(input, 2);
    println!("{}", part1);
}

fn get_counts(formula: String) -> usize {
    let sorted = formula.chars().collect::<Vec<char>>();
    sorted.sort();
    let counts = sorted
        .iter()
        .dedup_with_count()
        .sorted_by(|a, b| Ord::cmp(&b.0, &a.0));
}

fn iterate(input: Input, n: usize) -> String {
    (0..n)
        .into_iter()
        .fold(input.formula.to_owned(), |iter, _| {
            grow(&input.pairs, &iter)
        })
}

fn grow(pairs: &Pairs, formula: &str) -> String {
    let firsts = formula.chars();
    let seconds = {
        let mut ret = formula.chars();
        ret.next();
        ret
    };
    let mut ret = zip(firsts, seconds)
        .map(|(first, second)| {
            let pair: String = vec![first, second].iter().collect();
            println!("{}", pair);
            let insert = pairs.get(&pair[..]).unwrap();
            vec![first, *insert].iter().collect()
        })
        .collect::<Vec<String>>();
    ret.push(formula.chars().last().unwrap().to_string());
    ret.join("")
}

fn get_input(f: &str) -> Input {
    let mut input = f.split('\n').filter(|line| line.trim() != "");
    let formula = input.next().unwrap();
    let pairs = input.fold(HashMap::new(), |mut m: Pairs, line| {
        let r: Vec<&str> = line.split(" -> ").collect();
        m.insert(r[0], r[1].chars().next().unwrap());
        m
    });
    Input { formula, pairs }
}
