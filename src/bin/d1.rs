use std::fs;

fn main() {
    let f = fs::read_to_string("d1.txt").expect("no file");
    let v = f
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("number expected"));
    let mut count = -1;
    let mut prev = -1;
    v.for_each(|x| {
        if prev < x {
            count += 1;
        }
        prev = x;
    });
    let part1 = count;
    println!("{}", part1);

    count = 0;
    let mut window = Vec::new();
    for x in f.split_whitespace() {
        let x: i32 = x.parse().unwrap();
        window.push(x);
        if window.len() == 4 {
            let first: i32 = window[0..3].iter().sum();
            let second: i32 = window[1..4].iter().sum();
            if first < second {
                count += 1;
            }
            window.remove(0);
        }
    }
    println!("{}", count);
}
