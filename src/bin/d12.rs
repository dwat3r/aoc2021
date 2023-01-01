// use std::fs;
use std::collections::HashMap;

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

    let input = get_input(f);
}

fn get_input(f: &str) -> Input {
    f.split_whitespace()
        .map(|line| {
            let r: Vec<&str> = line.split('-').collect();
            (r[0], r[1])
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

fn list_paths(input: Input) -> Vec<Vec<&str>> {
    input
}
