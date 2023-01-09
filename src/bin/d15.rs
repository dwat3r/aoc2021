// use std::fs;

type Input = Vec<Vec<u32>>;

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

    let input = get_input(f);
    println!("{:?}", &input);
}

fn find_shortest_path(input: &Input, path_weight: u32, x: usize, y: usize) -> u32 {
    vec![x-1, x+1].iter().
    let w = input.get(x + 1).map(|l| l.get(y)).flatten();
    let e = input.get(x - 1).map(|l| l.get(y)).flatten();
    let s = input.get(x).map(|l| l.get(y + 1)).flatten();
    let n = input.get(x).map(|l| l.get(y - 1)).flatten();

}

fn get_input(f: &str) -> Input {
    f.split('\n')
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}
