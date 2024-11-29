#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal {
        version: u8,
        type_id: u8,
        value: u32,
    },
    Operator {
        version: u8,
        type_id: u8,
        sub_packets: Box<[Packet]>,
    },
}
use Packet::*;

fn main() {}

fn parse_packet(f: &str) -> Packet {
    let bits = f
        .as_bytes()
        .iter()
        .flat_map(|c| num_to_bits(&ascii_to_num(c)))
        .collect::<Vec<_>>();
    println!("{:?}", bits);

    let version = bits_to_num(&bits[0..3]) as u8;
    let type_id = bits_to_num(&bits[3..6]) as u8;

    if type_id == 4 {
        let (value, _) = get_literal(&bits[6..]);
        Literal {
            version,
            type_id,
            value,
        }
    } else {
        Operator {
            version,
            type_id,
            sub_packets: Box::new([Literal {
                version,
                type_id,
                value: 0,
            }]),
        }
    }
}

fn ascii_to_num(c: &u8) -> u8 {
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

fn bits_to_num(bits: &[u8]) -> u32 {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |num, (i, bit)| num + ((*bit as u32) << i))
}

// returns (value, remaining bits)
fn get_literal(bits: &[u8]) -> (u32, Vec<u8>) {
    let (nums, remaining) = bits.chunks(5).fold(
        (Vec::new(), Vec::new()),
        |(mut nums, mut remaining), chunk| {
            nums.push(bits_to_num(&chunk[1..]));
            if chunk[0] == 0 {
                remaining.push(&chunk[1..]);
                (nums, remaining)
            } else {
                (nums, remaining)
            }
        },
    );

    let num = nums.iter().rev().enumerate().fold(0, |num, (i, num_part)| {
        num + ((*num_part as u32) << (i * 4))
    });

    (num, remaining.concat())
}

fn get_subpackets(bits: &[u8]) -> &[Packet] {
    if bits[0] == 0 {
        let length = bits_to_num(&bits[1..16]);
    }
    &[]
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
    fn get_literal_test() {
        assert_eq!(
            get_literal(&[1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]),
            (2021, vec![0, 0, 0])
        )
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

    #[test]
    fn operator_1() {
        let f = "38006F45291200";
        let operator = parse_packet(f);
        assert_eq!(
            operator,
            Operator {
                version: 1,
                type_id: 6,
                sub_packets: Box::new([
                    Literal {
                        version: 0,
                        type_id: 4,
                        value: 10
                    },
                    Literal {
                        version: 0,
                        type_id: 4,
                        value: 20
                    }
                ])
            }
        )
    }
}
