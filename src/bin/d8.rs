use std::{fs, ops::RangeBounds};

fn main() {
    let f = fs::read_to_string("d8.txt")
        .expect("no file")
        .trim()
        .to_string();
    let f = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    let input: Vec<(Vec<&str>, Vec<&str>)> = f
        .lines()
        .map(|line| {
            let line: Vec<&str> = line
                .split_whitespace()
                .filter(|word| !word.contains("|"))
                .collect();
            (
                line.clone().into_iter().take(10).collect(),
                line.clone().into_iter().skip(10).collect(),
            )
        })
        .collect();

    let part1 = input
        .iter()
        .flat_map(|(_, outs)| {
            outs.iter()
                .filter(|w| [2, 3, 4, 7].iter().any(|&l| l == w.len() as i32))
        })
        .count();
    // println!("{}", part1);
    let first_line = input.iter().take(1).next().unwrap();
    println!("{}", decode_line(first_line));
}

fn decode_line(line: &(Vec<&str>, Vec<&str>)) -> i32 {
    let (ins, outs) = line;
    let mut positions: Vec<Vec<char>> = vec![('a'..='g').collect(); 7];
    let get_digits = |len| -> Vec<Vec<char>> {
        ins.iter()
            .filter(|w| w.len() == len)
            .map(|w| w.chars().collect())
            .collect()
    };
    let diff = |v1: &Vec<char>, v2: &Vec<char>| -> Vec<char> {
        v1.iter().filter(|e| !v2.contains(e)).cloned().collect()
    };

    let match_to = |digit| 

    let one: Vec<char> = get_digits(2)[0];
    positions[2] = one.clone();
    positions[5] = one.clone();
    let seven: Vec<char> = get_digits(3)[0];
    positions[0] = diff(&seven, &one);
    let four: Vec<char> = get_digits(4)[0];
    positions[1] = diff(&four, &one);
    positions[3] = positions[1].clone();

    positions[4] = diff(&positions[4], &vec![one, four, seven].concat());
    positions[6] = positions[4].clone();
    println!("{:?}", positions);
    let zero = get_digits(6);
    positions[]
    0
}
