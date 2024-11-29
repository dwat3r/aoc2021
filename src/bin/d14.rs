use std::{collections::HashMap, fs, iter::zip};

use itertools::Itertools;

type Pairs<'a> = HashMap<&'a str, char>;
type Formula = HashMap<String, u64>;

#[derive(Debug)]
struct Input<'a> {
    formula_str: &'a str,
    formula: Formula,
    pairs: Pairs<'a>,
}

fn main() {
    let f = fs::read_to_string("d14.txt").expect("no file");
    //     let f = "NNCB

    // CH -> B
    // HH -> N
    // CB -> H
    // NH -> C
    // HB -> C
    // HC -> B
    // HN -> C
    // NN -> C
    // BH -> H
    // NC -> B
    // NB -> B
    // BN -> B
    // BB -> N
    // BC -> B
    // CC -> N
    // CN -> C";

    //     let f = "BB

    // BB -> B";

    let input = get_input(&f);
    println!("{:?}", input);
    let part1 = get_counts(iterate(&input, 10));
    println!("{}", part1);
    let input = get_input(&f);
    let part2 = get_counts2(&input, iterate2(&input, 40));
    println!("{}", part2);
}

fn get_counts2(input: &Input, formula: Formula) -> u64 {
    let char_counts = formula.iter().fold(
        HashMap::new(),
        |mut m: HashMap<char, u64>, (pair, count)| {
            let pair = pair.chars().collect::<Vec<_>>();
            m.entry(pair[0])
                .and_modify(|c| *c += count)
                .or_insert(*count);
            m.entry(pair[1])
                .and_modify(|c| *c += count)
                .or_insert(*count);
            m
        },
    );

    let counts = char_counts
        .iter()
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        .collect::<Vec<_>>();

    let formula_str = input.formula_str.chars().collect::<Vec<_>>();
    let add_if_edge = |pair: (&char, &u64)| -> u64 {
        if *pair.0 == formula_str[0] || *pair.0 == formula_str[formula_str.len() - 1] {
            pair.1 / 2 + 1
        } else {
            pair.1 / 2
        }
    };
    add_if_edge(counts[0]) - add_if_edge(*counts.last().unwrap())
}

fn iterate2(input: &Input, n: usize) -> Formula {
    (0..n).fold(input.formula.clone(), |iter, _| grow2(&input.pairs, &iter))
}

fn grow2(pairs: &Pairs, formula: &Formula) -> Formula {
    let mut new_formula = HashMap::new();
    // println!("{:?}", formula);
    formula.iter().for_each(|(pair, count)| {
        let insert = pairs.get(pair.as_str()).unwrap();
        let pair = pair.chars().collect::<Vec<_>>();
        let first = [pair[0], *insert].iter().collect::<String>();
        let second = [*insert, pair[1]].iter().collect::<String>();
        new_formula
            .entry(first)
            .and_modify(|c| *c += count)
            .or_insert(*count);
        new_formula
            .entry(second)
            .and_modify(|c| *c += count)
            .or_insert(*count);
    });
    new_formula
}

fn get_input(f: &str) -> Input {
    let mut input = f.split('\n').filter(|line| line.trim() != "");
    let formula_str = input.next().unwrap();
    let formula = build_formula(formula_str);
    let pairs = input.fold(HashMap::new(), |mut m: Pairs, line| {
        let r: Vec<&str> = line.split(" -> ").collect();
        m.insert(r[0], r[1].chars().next().unwrap());
        m
    });
    Input {
        formula,
        formula_str,
        pairs,
    }
}

fn build_formula(input: &str) -> Formula {
    let firsts = input.chars().take(input.len() - 1);
    let seconds = input.chars().dropping(1);
    HashMap::from_iter(
        zip(firsts, seconds).map(|(first, second)| ([first, second].iter().collect(), 1)),
    )
}

fn get_counts(formula: String) -> usize {
    let mut sorted = formula.chars().collect::<Vec<char>>();
    sorted.sort();
    let counts = sorted
        .iter()
        .dedup_with_count()
        .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
        .collect::<Vec<_>>();
    counts[0].0 - counts.last().unwrap().0
}

fn iterate(input: &Input, n: usize) -> String {
    (0..n).fold(input.formula_str.to_owned(), |iter, _| {
        grow(&input.pairs, &iter)
    })
}

fn grow(pairs: &Pairs, formula: &str) -> String {
    let firsts = formula.chars();
    let seconds = formula.chars().dropping(1);
    let mut ret = zip(firsts, seconds)
        .map(|(first, second)| {
            let pair: String = [first, second].iter().collect();
            // println!("{}", pair);
            let insert = pairs.get(&pair[..]).unwrap();
            [first, *insert].iter().collect()
        })
        .collect::<Vec<String>>();
    ret.push(formula.chars().last().unwrap().to_string());
    ret.join("")
}
