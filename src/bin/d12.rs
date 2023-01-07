// use std::fs;
use std::{collections::HashMap, fs, ops::Not};

use itertools::Itertools;

type Input<'a> = HashMap<&'a str, Vec<&'a str>>;

fn main() {
    let f = fs::read_to_string("d12.txt").expect("no file");
    //     let f = "start-A
    // start-b
    // A-c
    // A-b
    // b-d
    // A-end
    // b-end";

    //     let f = "start-A
    // start-b
    // A-b
    // A-end
    // b-end";
    //         let f = "start-A
    // A-end";
    //     let f = "start-A
    // A-end";
    // let f = "start-end";
    //     let f = "dc-end
    // HN-start
    // start-kj
    // dc-start
    // dc-HN
    // LN-dc
    // HN-end
    // kj-sa
    // kj-HN
    // kj-dc";
    //     let f = "fs-end
    // he-DX
    // fs-he
    // start-DX
    // pj-DX
    // end-zg
    // zg-sl
    // zg-pj
    // pj-he
    // RW-he
    // fs-DX
    // pj-RW
    // zg-RW
    // start-pj
    // he-WI
    // zg-he
    // pj-fs
    // start-RW";
    let input = get_input(&f);
    println!("{:?}", &input);
    let part1 = list_paths(&input, "start", &vec!["start"])
        .iter()
        .map(|path| path.join(","))
        .collect::<Vec<String>>();

    println!("{}\n{}", &part1.join("\n"), &part1.len());

    let part2 = list_paths2(&input, "start", &vec!["start"])
        .iter()
        .map(|path| path.join(","))
        .collect::<Vec<String>>();
    println!("{}\n{}", &part2.join("\n"), &part2.len());
}

fn get_input(f: &str) -> Input {
    f.split_whitespace()
        .flat_map(|line| {
            let r: Vec<&str> = line.split('-').collect();
            if r[0] != "start" && r[1] != "end" {
                vec![(r[0], r[1]), (r[1], r[0])]
            } else {
                vec![(r[0], r[1])]
            }
        })
        .fold(HashMap::new(), |mut m: Input, (from, to)| {
            let tos: Vec<&str> = if let Some(tos) = m.get_mut(from) {
                tos.push(to);
                tos.to_vec()
            } else {
                vec![to]
            };
            m.insert(from, tos);
            m
        })
}

/*
start -> [A -> [c, b, end], b -> [d, A]]
1.
start, A
start, b
2.
start, A, end
start, A, c,
start, A, b
start, b, A,
...
*/

fn list_paths<'a>(input: &Input<'a>, from: &'a str, init: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
    if from == "end" {
        return vec![init.clone()];
    }
    let no_go = init
        .iter()
        .cloned()
        .filter(|x| x.chars().any(|x| x.is_lowercase()))
        .collect::<Vec<&str>>();
    let tos: Vec<&str> = input
        .get(from)
        .unwrap()
        .iter()
        .cloned()
        .filter(|to| no_go.contains(to).not())
        .collect();

    // println!("{:?} {:?}", init, tos);
    let ret = tos
        .iter()
        .flat_map(|to| {
            // println!("{:?} {:?} {}", no_go, path, to);
            let mut new_init = init.clone();
            new_init.push(to);
            let ret = list_paths(input, to, &new_init);
            ret
        })
        .collect();
    ret
}

fn list_paths2<'a>(input: &Input<'a>, from: &'a str, init: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
    if from == "end" {
        return vec![init.clone()];
    }
    let small_caves = init
        .iter()
        .cloned()
        .filter(|x| x.chars().any(|x| x.is_lowercase()))
        .sorted()
        .dedup_with_count()
        .fold(
            HashMap::new(),
            |mut m: HashMap<usize, Vec<&str>>, (size, cave)| {
                let caves: Vec<&str> = if let Some(caves) = m.get_mut(&size) {
                    caves.push(cave);
                    caves.to_vec()
                } else {
                    vec![cave]
                };
                m.insert(size, caves);
                m
            },
        );
    let no_go = init
        .iter()
        .cloned()
        .filter(|x| {
            let is_lowercase = x.chars().any(|x| x.is_lowercase());
            if *x == "start" {
                true
            } else if small_caves.get(&2).is_some() {
                is_lowercase
            } else if small_caves.get(&1).filter(|one| one.contains(x)).is_some() {
                false
            } else {
                is_lowercase
            }
        })
        .collect::<Vec<&str>>();
    let tos: Vec<&str> = input
        .get(from)
        .unwrap()
        .iter()
        .cloned()
        .filter(|to| no_go.contains(to).not())
        .collect();

    // println!("{:?} {:?}", init, tos);
    let ret = tos
        .iter()
        .flat_map(|to| {
            // println!("{:?} {:?} {}", no_go, path, to);
            let mut new_init = init.clone();
            new_init.push(to);
            let ret = list_paths2(input, to, &new_init);
            ret
        })
        .collect();
    ret
}
