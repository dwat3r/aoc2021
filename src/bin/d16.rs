use std::fs;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal {
        version: u8,
        type_id: u8,
        value: u64,
    },
    Operator {
        version: u8,
        type_id: u8,
        sub_packets: Vec<Packet>,
    },
}

use Packet::*;

fn main() {
    let f = fs::read_to_string("d16.txt").expect("no file");
    let packet = parse_packet(&f);
    let part1 = version_sum(&packet);
    println!("part1: {}", part1);

    let part2 = evaluate(&packet);
    println!("part2: {}", part2);
}

fn version_sum(packet: &Packet) -> u32 {
    match packet {
        Packet::Literal { version, .. } => *version as u32,
        Packet::Operator {
            sub_packets,
            version,
            ..
        } => *version as u32 + sub_packets.iter().map(version_sum).sum::<u32>(),
    }
}

fn evaluate(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { value, .. } => *value,
        Packet::Operator {
            sub_packets,
            type_id,
            ..
        } => match type_id {
            0 => sub_packets.iter().map(evaluate).sum(),
            1 => sub_packets.iter().map(evaluate).product(),
            2 => sub_packets.iter().map(evaluate).min().unwrap(),
            3 => sub_packets.iter().map(evaluate).max().unwrap(),
            5 => {
                let evald = sub_packets.iter().map(evaluate).collect_vec();
                if evald[0] > evald[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                let evald = sub_packets.iter().map(evaluate).collect_vec();
                if evald[0] < evald[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                let evald = sub_packets.iter().map(evaluate).collect_vec();
                if evald[0] == evald[1] {
                    1
                } else {
                    0
                }
            }
            x => panic!("packet type_id {} unkown", x),
        },
    }
}

fn parse_packet(f: &str) -> Packet {
    let bits = str_to_bits(f);
    get_packet(&bits).0
}

/*
first do it with vectors
then do it with iterators
*/

fn get_packet(bits: &[u8]) -> (Packet, u64) {
    // another exit condition is when version is all zeros
    let version = bits_to_num(&bits[0..3]) as u8;
    let type_id = bits_to_num(&bits[3..6]) as u8;
    // println!(
    //     "packet, version: {}, type_id: {},  bits: {}",
    //     version,
    //     type_id,
    //     bit_fmt(bits)
    // );

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
                sub_packets: packets,
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
fn get_literal(bits: &[u8]) -> (u64, u64) {
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

    // println!(
    //     "literal, consumed: {}, num: {} bits: {}",
    //     consumed,
    //     num,
    //     bit_fmt(bits)
    // );
    (num, consumed)
}

fn get_subpackets(bits: &[u8]) -> (Vec<Packet>, u64) {
    if bits[0] == 0 {
        /*
        00111000000000000110111101000101001010010001001000000000
        VVVTTTILLLLLLLLLLLLLLLAAAAAAAAAAABBBBBBBBBBBBBBBB
                              VVVTTTAAAAAVVVTTTAAAAABBBBB
        */
        let mut packets = Vec::new();
        let length = bits_to_num(&bits[1..16]);
        // println!("operator0, len: {}, bits: {},", length, bit_fmt(bits));
        let (p_packet, p_consumed) = get_packet(&bits[16..16 + length as usize]);
        packets.push(p_packet);
        let mut consumed = p_consumed;
        while consumed < length {
            let (p_packet, p_consumed) =
                get_packet(&bits[16 + consumed as usize..16 + length as usize]);
            packets.push(p_packet);
            consumed += p_consumed;
        }
        (packets, consumed + 16)
    } else {
        /*
        11101110000000001101010000001100100000100011000001100000
        VVVTTTILLLLLLLLLLLAAAAAAAAAAABBBBBBBBBBBCCCCCCCCCCC
         */
        let mut packets = Vec::new();
        let packet_count = bits_to_num(&bits[1..12]);
        // println!(
        //     "operator1 count: {}, bits: {},",
        //     packet_count,
        //     bit_fmt(bits)
        // );
        let (p_packet, p_consumed) = get_packet(&bits[12..]);
        packets.push(p_packet);
        let mut consumed = p_consumed;
        for i in 1..packet_count {
            // println!("operator1 iteration: {}, consumed: {}", i, consumed);
            let (p_packet, p_consumed) = get_packet(&bits[12 + consumed as usize..]);
            packets.push(p_packet);
            consumed += p_consumed;
        }
        (packets, consumed + 12)
    }
}

fn str_to_bits(f: &str) -> Vec<u8> {
    let bits = f
        .as_bytes()
        .iter()
        .flat_map(|c| num_to_bits(&ascii_to_num(c)))
        .collect::<Vec<_>>();
    println!("{}", bit_fmt(&bits));
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

fn bits_to_num(bits: &[u8]) -> u64 {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |num, (i, bit)| num + ((*bit as u64) << i))
}

fn bit_fmt(bits: &[u8]) -> String {
    bits.iter().join("").to_string()
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
                sub_packets: vec![
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
                ]
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
                sub_packets: vec![
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
                ]
            }
        )
    }
    #[test]
    fn operator_nested() {
        let f = "8A004A801A8002F478";
        let operator = parse_packet(f);
        assert_eq!(
            operator,
            Operator {
                version: 4,
                type_id: 2,
                sub_packets: vec![Operator {
                    version: 1,
                    type_id: 2,
                    sub_packets: vec![Operator {
                        version: 5,
                        type_id: 2,
                        sub_packets: vec![Literal {
                            version: 6,
                            type_id: 4,
                            value: 15
                        }]
                    }]
                },]
            }
        )
    }
    #[test]
    fn operator_tree() {
        let f = "620080001611562C8802118E34";
        /*
         011 000 1 00000000010 000 000 0 000000000010110 000100 01010 101100 01011 001000 1 00000000010 000100 01100 011100 01101 00
        operator1      size 2 operator0         size 22 literal   10 literal   11 operator1     size 2 literal   12 literal   13

                 */
        let operator = parse_packet(f);
        let sum = version_sum(&operator);
        assert_eq!(sum, 12);
        assert_eq!(
            operator,
            Operator {
                version: 3,
                type_id: 0,
                sub_packets: vec![
                    Operator {
                        version: 0,
                        type_id: 0,
                        sub_packets: vec![
                            Literal {
                                version: 0,
                                type_id: 4,
                                value: 10
                            },
                            Literal {
                                version: 5,
                                type_id: 4,
                                value: 11
                            }
                        ]
                    },
                    Operator {
                        version: 1,
                        type_id: 0,
                        sub_packets: vec![
                            Literal {
                                version: 0,
                                type_id: 4,
                                value: 12
                            },
                            Literal {
                                version: 3,
                                type_id: 4,
                                value: 13
                            }
                        ]
                    }
                ]
            }
        )
    }

    #[test]
    fn operator_tree_sum() {
        let f = "C0015000016115A2E0802F182340";
        /*
         110 000 1 00000000010 000 000 0 000000000010110 000100 01010 101100 01011 001000 1 00000000010 000100 01100 011100 01101 00
        operator1      size 2 operator0         size 22 literal   10 literal   11 operator1     size 2 literal   12 literal   13

                 */
        let operator = parse_packet(f);
        let sum = version_sum(&operator);
        assert_eq!(sum, 23)
    }
    #[test]
    fn operator_tree_sum_2() {
        let f = "A0016C880162017C3686B18A3D4780";
        let operator = parse_packet(f);
        let sum = version_sum(&operator);
        assert_eq!(sum, 31);
    }

    #[test]
    fn evaluate_1() {
        assert_eq!(evaluate(&parse_packet("C200B40A82")), 3);
    }

    #[test]
    fn evaluate_2() {
        assert_eq!(evaluate(&parse_packet("04005AC33890")), 54);
        assert_eq!(evaluate(&parse_packet("880086C3E88112")), 7);
        assert_eq!(evaluate(&parse_packet("CE00C43D881120")), 9);
        assert_eq!(evaluate(&parse_packet("D8005AC2A8F0")), 1);
        assert_eq!(evaluate(&parse_packet("F600BC2D8F")), 0);
        assert_eq!(evaluate(&parse_packet("9C005AC2F8F0")), 0);
        assert_eq!(evaluate(&parse_packet("9C0141080250320F1802104A08")), 1);
    }
}
