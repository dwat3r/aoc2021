use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

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
        sub_packets: Box<Vec<Packet>>,
    },
}

use Packet::*;

fn main() {}

fn parse_packet(f: &str) -> Packet {
    let bits = str_to_bits(f);
    get_packet(&bits).0
}

/*
first do it with vectors
then do it with iterators
*/

fn get_packet(bits: &[u8]) -> (Packet, u32) {
    // another exit condition is when version is all zeros
    let version = bits_to_num(&bits[0..3]) as u8;
    let type_id = bits_to_num(&bits[3..6]) as u8;
    println!(
        "packet, bits: {:?}, version: {}, type_id: {}",
        bits, version, type_id
    );

    if type_id == 4 {
        let (value, consumed) = get_literal(&bits[6..]);
        (
            Literal {
                version,
                type_id,
                value,
            },
            consumed + 6,
        )
    } else {
        let (packets, consumed) = get_subpackets(&bits[6..]);
        (
            Operator {
                version,
                type_id,
                sub_packets: Box::new(packets),
            },
            consumed + 6,
        )
    }
}
/*
110100101111111000101000
VVVTTTAAAAABBBBBCCCCC
*/
// returns (value, consumed bits)
fn get_literal(bits: &[u8]) -> (u32, u32) {
    let (nums, consumed) = bits
        .chunks(5)
        .fold_while((Vec::new(), 0), |(mut nums, consumed), chunk| {
            nums.push(bits_to_num(&chunk[1..]));
            if chunk[0] == 0 {
                Done((nums, consumed + 5))
            } else {
                Continue((nums, consumed + 5))
            }
        })
        .into_inner();

    let num = nums
        .iter()
        .rev()
        .enumerate()
        .fold(0, |num, (i, num_part)| num + (*num_part << (i * 4)));

    println!("literal, bits: {:?}, consumed: {}", bits, consumed);
    (num, consumed)
}

fn get_subpackets(bits: &[u8]) -> (Vec<Packet>, u32) {
    if bits[0] == 0 {
        /*
        00111000000000000110111101000101001010010001001000000000
        VVVTTTILLLLLLLLLLLLLLLAAAAAAAAAAABBBBBBBBBBBBBBBB
                              VVVTTTAAAAAVVVTTTAAAAABBBBB
        */
        let mut packets = Vec::new();
        let length = bits_to_num(&bits[1..16]);
        println!("bits: {:?}, len: {}", bits, length);
        let (p_packet, p_consumed) = get_packet(&bits[16..]);
        packets.push(p_packet);
        let mut consumed = p_consumed;
        while consumed < length {
            let (p_packet, p_consumed) = get_packet(&bits[16 + consumed as usize..]);
            packets.push(p_packet);
            consumed += p_consumed;
        }
        (packets, consumed)
    } else {
        /*
        11101110000000001101010000001100100000100011000001100000
        VVVTTTILLLLLLLLLLLAAAAAAAAAAABBBBBBBBBBBCCCCCCCCCCC
         */
        let mut packets = Vec::new();
        let packet_count = bits_to_num(&bits[1..12]);
        let (p_packet, p_consumed) = get_packet(&bits[12..]);
        packets.push(p_packet);
        let mut consumed = p_consumed;
        for _ in 1..packet_count {
            let (p_packet, p_consumed) = get_packet(&bits[12 + consumed as usize..]);
            packets.push(p_packet);
            consumed += p_consumed;
        }
        (packets, consumed)
    }
}

fn str_to_bits(f: &str) -> Vec<u8> {
    let bits = f
        .as_bytes()
        .iter()
        .flat_map(|c| num_to_bits(&ascii_to_num(c)))
        .collect::<Vec<_>>();
    println!("{:?}", bits);
    bits
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
            (2021, 15)
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
    fn operator_length_type_0() {
        let f = "38006F45291200";
        let operator = parse_packet(f);
        assert_eq!(
            operator,
            Operator {
                version: 1,
                type_id: 6,
                sub_packets: Box::new(vec![
                    Literal {
                        version: 6,
                        type_id: 4,
                        value: 10
                    },
                    Literal {
                        version: 2,
                        type_id: 4,
                        value: 20
                    }
                ])
            }
        )
    }

    #[test]
    fn operator_length_type_1() {
        let f = "EE00D40C823060";
        let operator = parse_packet(f);
        assert_eq!(
            operator,
            Operator {
                version: 7,
                type_id: 3,
                sub_packets: Box::new(vec![
                    Literal {
                        version: 2,
                        type_id: 4,
                        value: 1
                    },
                    Literal {
                        version: 4,
                        type_id: 4,
                        value: 2
                    },
                    Literal {
                        version: 1,
                        type_id: 4,
                        value: 3
                    }
                ])
            }
        )
    }
    // fn operator_nested()
}
