// use std::fs;

use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Node {
    weight: u32,
    dist: u32,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entry(&self.weight)
            .entry(&self.dist)
            .finish()
    }
}

type Graph = HashMap<(i32, i32), Node>;

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
    let mut shortests: Graph = HashMap::new();
    // todo: use a binary heap
    let mut queue = input.clone();

    queue.get_mut(&(0, 0)).unwrap().dist = 0;

    while !queue.is_empty() {
        let (upos, unode) = queue
            .clone()
            .into_iter()
            .min_by(|a, b| a.1.dist.cmp(&b.1.dist))
            .unwrap();

        queue.remove(&upos);
        shortests.insert(upos, unode);

        let neighs: Vec<_> = [(1_i32, 0_i32), (0, 1), (-1, 0), (0, -1)]
            .iter()
            .filter(|pos| queue.contains_key(&(upos.0 + pos.0, upos.1 + pos.1)))
            .collect();
        println!("neighs: {:?}", &neighs);
        println!("queue: {:?}", &queue);

        for neigh in neighs {
            let qneigh = queue.get_mut(neigh).unwrap();
            let alt = unode.dist + qneigh.weight;
            if alt < qneigh.dist {
                qneigh.dist = alt;
            }
        }
        println!("----");
    }
    let n = num::integer::sqrt(input.len() as i32);
    pretty_graph(&shortests);

    shortests.get(&(n - 1, n - 1)).unwrap().dist
}

fn get_input(f: &str) -> Graph {
    let graph_size = f.len() as u32;
    f.split('\n')
        .enumerate()
        .flat_map(|(y, xs)| {
            xs.chars().enumerate().map(move |(x, w)| {
                (
                    (x as i32, y as i32),
                    Node {
                        weight: (w.to_digit(10).unwrap()),
                        dist: graph_size * 10,
                    },
                )
            })
        })
        .collect::<HashMap<(i32, i32), Node>>()
}

fn pretty_graph(graph: &Graph) {
    let mut drawing = String::new();
    let n = num::integer::sqrt(graph.len());
    for y in 0..n {
        for x in 0..n {
            drawing = format!("{}{:?};", drawing, graph[&(x as i32, y as i32)]);
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
    // #[test]
    //     fn basic2() {
    //         let f = "116
    // 138
    // 213";
    //         assert_eq!(5, part1(f));
    //     }
}
