use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Literal {
    version: u8,
    type_id: u8,
    value: u32,
}

fn main() {}

fn parse_packet(f: &str) -> Literal {
    let bits = f
        .as_bytes()
        .iter()
        .flat_map(|c| num_to_bits(&ascii_to_num(c)))
        .collect::<Vec<_>>();
    println!("{:?}", bits);

    let version = bits_to_num(&bits[0..3]);
    let type_id = bits_to_num(&bits[3..6]);

    Literal {
        version,
        type_id,
        value: 0,
    }
}

fn ascii_to_num(c: &u8) -> u8 {
    println!("{}", c);
    if c.is_ascii_digit() {
        c - 48
    } else {
        c - 55
    }
}

fn num_to_bits(hex: &u8) -> Vec<u8> {
    let mut ret = Vec::new();
    let mut n = *hex;
    for _ in 0..4 {
        let x = n & 0b0000_0001;
        ret.push(x);
        n >>= 1;
    }
    ret.reverse();
    ret
}

fn bits_to_num(bits: &[u8]) -> u8 {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |num, (i, bit)| num + (*bit << i))
}

fn get_literal(bits: &[u8]) -> u32 {
    for mut chunk in &bits.into_iter().chunks(5) {
        let read_more = chunk.next().unwrap();
        let num = bits_to_num(&chunk.collect_vec()[..]);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_to_bits_test() {
        assert_eq!(num_to_bits(&13), vec![1, 1, 0, 1]);
    }

    #[test]
    fn bits_to_num_test() {
        assert_eq!(bits_to_num(&[1, 1, 0, 1]), 13);
        assert_eq!(bits_to_num(&[1, 1, 0]), 6);
    }
    #[test]
    fn literal() {
        let f = "D2FE28";
        let literal = parse_packet(f);
        assert_eq!(
            literal,
            Literal {
                version: 6,
                type_id: 4,
                value: 2021
            }
        )
    }
}
