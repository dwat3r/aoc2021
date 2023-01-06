// use std::fs;
use std::{collections::HashMap, ops::Not};

type Input<'a> = HashMap<&'a str, Vec<&'a str>>;

fn main() {
    // let f = fs::read_to_string("d12.txt").expect("no file");
    let f = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

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
    let input = get_input(f);
    println!("{:?}", &input);
    let part1 = list_paths(&input, "start", &vec!["start"]);
    println!("{:?}", &part1);
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
        .filter(|x| x.chars().any(|x| x.is_lowercase()) || *x != "end")
        .collect::<Vec<&str>>();
    let tos: Vec<&str> = input
        .get(from)
        .unwrap()
        .iter()
        .cloned()
        .filter(|to| no_go.contains(to).not())
        .collect();

    println!("{:?} {:?}", init, tos);
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
