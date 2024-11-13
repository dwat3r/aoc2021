// use std::fs;

use std::collections::{HashMap, HashSet};

struct Node {
    weight: u32,
    path: u32,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entry(&self.weight)
            .entry(&self.path)
            .finish()
    }
}

type Graph = HashMap<(usize, usize), Node>;

fn main() {
    // let f = fs::read_to_string("d15.txt").expect("no file");
    let f = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    let part1 = part1(f);
    println!("{:?}", part1);
}

fn part1(f: &str) -> u32 {
    let graph = get_input(f);
    pretty_graph(&graph);
    find_shortest_path(&graph)
}

fn find_shortest_path(input: &Graph) -> u32 {
    let (mut xi, mut yi) = (0, 0);

    0
}

fn get_input(f: &str) -> Graph {
    f.split('\n')
        .enumerate()
        .flat_map(|(y, xs)| {
            xs.chars().enumerate().map(move |(x, w)| {
                (
                    (x, y),
                    Node {
                        weight: (w.to_digit(10).unwrap()),
                        path: 0,
                    },
                )
            })
        })
        .collect::<HashMap<(usize, usize), Node>>()
}

fn pretty_graph(graph: &Graph) {
    let mut drawing = String::new();
    let n = num::integer::sqrt(graph.len());
    for y in 0..n {
        for x in 0..n {
            drawing = format!("{}{:?};", drawing, graph[&(x, y)]);
        }
        drawing.push('\n');
    }
    println!("{}", drawing);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let f = "12
12";

        assert_eq!(3, part1(f));
    }
    #[test]
    fn basic2() {
        let f = "116
138
213";
        assert_eq!(5, part1(f));
    }
}
