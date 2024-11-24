use std::cmp::Reverse;
use std::fs;

use std::collections::HashMap;

use priority_queue::PriorityQueue;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
    let f = fs::read_to_string("d15.txt").expect("no file");

    let part1 = part1(&f);
    println!("{:?}", part1);
    let part2 = part2(&f);
    println!("part2: {:?}", part2);
}

fn part1(f: &str) -> i32 {
    let graph = get_input(f);
    // pretty_graph(&graph);
    find_shortest_path(&graph)
}

fn part2(f: &str) -> i32 {
    let graph = get_input(f);
    let multiplied = multiply_input(&graph, 5);
    find_shortest_path(&multiplied)
}

fn find_shortest_path(input: &Graph) -> i32 {
    let mut dists = HashMap::new();
    let mut queue = PriorityQueue::new();
    let n = get_width(input);

    input.keys().for_each(|pos| {
        queue.push(*pos, Reverse(n * n * 10));
        dists.insert(*pos, n * n * 10);
    });
    queue.push((0, 0), Reverse(0));
    dists.insert((0, 0), 0);

    let mut i = 0;
    while !queue.is_empty() {
        if i % 100 == 0 {
            println!(".");
            i += 1;
        };
        let (upos, _) = queue.pop().unwrap();

        let neighs = [(1_i32, 0_i32), (0, 1), (-1, 0), (0, -1)];
        // println!(
        //     "upos: {:?}, neighs: {:?}, queue: {:?}, shortests: {:?}",
        //     upos, &neighs, &queue, &shortests
        // );

        for npos in neighs {
            let vpos = &(upos.0 + npos.0, upos.1 + npos.1);
            let udist = dists.get(&upos).unwrap();
            let vdisto = dists.get(vpos);
            if vdisto.is_none() {
                continue;
            }
            let vdist = *vdisto.unwrap();
            let vweight = input.get(vpos).unwrap().weight as i32;

            let alt = udist + vweight;
            if alt < vdist {
                dists.insert(*vpos, alt);
                queue.push_decrease(*vpos, Reverse(alt));
            }
        }
        // println!("----");
        // if upos == (n - 1, n - 1) {
        //     break;
        // }
    }

    *dists.get(&(n - 1, n - 1)).unwrap()
}

fn get_width(graph: &Graph) -> i32 {
    num::integer::sqrt(graph.len() as i32)
}

fn get_input(f: &str) -> Graph {
    let graph_size = num::pow(f.split('\n').count(), 2) as u32;
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

fn multiply_input(graph: &Graph, multiplier: i32) -> Graph {
    let n = get_width(graph);
    let infinite = (num::pow(n * multiplier, 2)) * 10;
    graph
        .iter()
        .flat_map(|(pos, node)| {
            (0..multiplier)
                .flat_map(|x| {
                    (0..multiplier).map(move |y| {
                        (
                            (pos.0 + n * x, pos.1 + n * y),
                            Node {
                                dist: infinite as u32,
                                weight: (node.weight + x as u32 + y as u32 - 1) % 9 + 1,
                            },
                        )
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn pretty_graph(graph: &Graph, show_path: bool) {
    let mut drawing = String::new();
    let n = get_width(graph);
    for y in 0..n {
        for x in 0..n {
            drawing = format!(
                "{}{},{};",
                drawing,
                graph[&(x, y)].weight,
                if show_path {
                    graph[&(x, y)].dist.to_string()
                } else {
                    "".to_string()
                }
            );
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
        assert_eq!(7, part1(f));
    }

    #[test]
    fn example() {
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
        assert_eq!(part1(f), 40);
    }

    #[test]
    fn multiply_one() {
        let f = get_input("8");

        let multiplied = multiply_input(&f, 5);
        pretty_graph(&multiplied, true);

        pretty_graph(
            &get_input(
                "89123
91234
12345
23456
34567",
            ),
            true,
        );
        assert_eq!(
            multiplied,
            get_input(
                "89123
91234
12345
23456
34567"
            )
        );
    }
    #[test]
    fn multiply_example() {
        let example_f = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        let example_multiplied_f = "11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479";

        let example = get_input(example_f);
        let multiplied = multiply_input(&example, 5);
        pretty_graph(&multiplied, false);
        let correct = get_input(example_multiplied_f);
        assert_eq!(multiplied, correct);
    }

    #[test]
    fn part2_example() {
        let example_f = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        let part2 = part2(example_f);
        assert_eq!(part2, 315);
    }
}
