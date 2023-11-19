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
    let part1 = find_shortest_path(&input, 0, 0, 0);
    println!("{:?}", part1);
}

fn find_shortest_path(input: &Input, path_weight: u32, x: usize, y: usize) -> u32 {
    if x == input.len() - 1 && y == input.len() - 1 {
        return path_weight + input[x][y];
    }

    let (nx, ny, w) = vec![
        x.checked_sub(1).map(|x| (x, y)),
        y.checked_sub(1).map(|y| (x, y)),
        Some((x + 1, y)),
        Some((x, y + 1)),
    ]
    .into_iter()
    .flatten()
    .flat_map(|(x, y)| {
        input
            .get(x)
            .and_then(|l| l.get(y))
            .map(|w| (x, y, path_weight + w))
    })
    .max_by(|a, b| a.2.cmp(&b.2))
    .unwrap();

    find_shortest_path(input, w, nx, ny)
}

fn get_input(f: &str) -> Input {
    f.split('\n')
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}
