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

    println!(
        "literal, consumed: {}, num: {}, bits: {}",
        consumed,
        num,
        bit_fmt(bits)
    );
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
        println!("operator0, len: {}, bits: {},", length, bit_fmt(bits));
        let (p_packet, p_consumed) = get_packet(&bits[16..]);
        packets.push(p_packet);
        let mut consumed = p_consumed;
        while consumed < length {
            let (p_packet, p_consumed) = get_packet(&bits[16 + consumed as usize..]);
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
        println!(
            "operator1 count: {}, bits: {},",
            packet_count,
            bit_fmt(bits)
        );
        let (p_packet, p_consumed) = get_packet(&bits[12..]);
        packets.push(p_packet);
        let mut consumed = p_consumed;
        for i in 1..packet_count {
            println!("operator1 iteration: {}, consumed: {}", i, consumed);
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

fn bits_to_num(bits: &[u8]) -> u32 {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |num, (i, bit)| num + ((*bit as u32) << i))
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
    #[test]
    fn operator_nested() {
        let f = "8A004A801A8002F478";
        let operator = parse_packet(f);
        assert_eq!(
            operator,
            Operator {
                version: 4,
                type_id: 2,
                sub_packets: Box::new(vec![Operator {
                    version: 1,
                    type_id: 2,
                    sub_packets: Box::new(vec![Operator {
                        version: 5,
                        type_id: 2,
                        sub_packets: Box::new(vec![Literal {
                            version: 6,
                            type_id: 4,
                            value: 15
                        }])
                    }])
                },])
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
        assert_eq!(
            operator,
            Operator {
                version: 3,
                type_id: 0,
                sub_packets: Box::new(vec![
                    Operator {
                        version: 0,
                        type_id: 0,
                        sub_packets: Box::new(vec![
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
                        ])
                    },
                    Operator {
                        version: 1,
                        type_id: 0,
                        sub_packets: Box::new(vec![
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
                        ])
                    }
                ])
            }
        )
    }
}
