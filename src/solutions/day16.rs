use crate::solutions::Solution;
extern crate hex;

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal {
        version: u64,
        value: i64,
    },
    Op {
        version: u64,
        operation: Operation,
        sub_packets: Vec<Packet>,
    },
}

#[inline]
fn read_bit(bytes: &[u8], i: usize) -> bool {
    let byte = i / 8;
    let bit = 7 - (i % 8);

    (bytes[byte] & (1 << bit)) != 0
}

#[inline]
fn read_triplet(bytes: &[u8], i: usize) -> u8 {
    ((read_bit(bytes, i) as u8) << 2)
        | ((read_bit(bytes, i + 1) as u8) << 1)
        | (read_bit(bytes, i + 2) as u8)
}

#[inline]
fn read_five(bytes: &[u8], i: usize) -> u8 {
    ((read_bit(bytes, i) as u8) << 4)
        | ((read_bit(bytes, i + 1) as u8) << 3)
        | ((read_bit(bytes, i + 2) as u8) << 2)
        | ((read_bit(bytes, i + 3) as u8) << 1)
        | (read_bit(bytes, i + 4) as u8)
}

#[inline]
fn read_eleven(bytes: &[u8], i: usize) -> u16 {
    ((read_bit(bytes, i) as u16) << 10)
        | ((read_bit(bytes, i + 1) as u16) << 9)
        | ((read_bit(bytes, i + 2) as u16) << 8)
        | ((read_bit(bytes, i + 3) as u16) << 7)
        | ((read_bit(bytes, i + 4) as u16) << 6)
        | ((read_bit(bytes, i + 5) as u16) << 5)
        | ((read_bit(bytes, i + 6) as u16) << 4)
        | ((read_bit(bytes, i + 7) as u16) << 3)
        | ((read_bit(bytes, i + 8) as u16) << 2)
        | ((read_bit(bytes, i + 9) as u16) << 1)
        | (read_bit(bytes, i + 10) as u16)
}

#[inline]
fn read_fifteen(bytes: &[u8], i: usize) -> u16 {
    ((read_bit(bytes, i) as u16) << 14)
        | ((read_bit(bytes, i + 1) as u16) << 13)
        | ((read_bit(bytes, i + 2) as u16) << 12)
        | ((read_bit(bytes, i + 3) as u16) << 11)
        | ((read_bit(bytes, i + 4) as u16) << 10)
        | ((read_bit(bytes, i + 5) as u16) << 9)
        | ((read_bit(bytes, i + 6) as u16) << 8)
        | ((read_bit(bytes, i + 7) as u16) << 7)
        | ((read_bit(bytes, i + 8) as u16) << 6)
        | ((read_bit(bytes, i + 9) as u16) << 5)
        | ((read_bit(bytes, i + 10) as u16) << 4)
        | ((read_bit(bytes, i + 11) as u16) << 3)
        | ((read_bit(bytes, i + 12) as u16) << 2)
        | ((read_bit(bytes, i + 13) as u16) << 1)
        | (read_bit(bytes, i + 14) as u16)
}

fn parse_literal(bytes: &[u8], start: usize, version: u64) -> (Packet, usize) {
    let mut index = start;
    let mut value: i64 = 0;
    loop {
        let nibble = read_five(bytes, index);
        index += 5;

        value += (nibble & 0b1111) as i64;

        if (nibble & 0b10000) != 0 {
            value <<= 4;
        } else {
            break;
        }
    }

    (Packet::Literal { version, value }, index)
}

fn operation_from_type(type_id: u8) -> Operation {
    match type_id {
        0 => Operation::Sum,
        1 => Operation::Product,
        2 => Operation::Minimum,
        3 => Operation::Maximum,
        5 => Operation::GreaterThan,
        6 => Operation::LessThan,
        7 => Operation::EqualTo,
        _ => panic!("Unknown op"),
    }
}

fn parse_count_op(bytes: &[u8], start: usize, version: u64, type_id: u8) -> (Packet, usize) {
    let mut index = start;
    let count = read_eleven(bytes, index);
    index += 11;

    let mut sub_packets = vec![];

    for _ in 0..count {
        let parsed = parse_packet(bytes, index);

        index = parsed.1;
        sub_packets.push(parsed.0);
    }

    (
        Packet::Op {
            version,
            operation: operation_from_type(type_id),
            sub_packets,
        },
        index,
    )
}

fn parse_length_op(bytes: &[u8], start: usize, version: u64, type_id: u8) -> (Packet, usize) {
    let mut index = start;
    let length = read_fifteen(bytes, index) as usize;
    index += 15;

    let mut sub_packets = vec![];

    while index < start + length + 15 {
        let parsed = parse_packet(bytes, index);

        index = parsed.1;
        sub_packets.push(parsed.0);
    }

    (
        Packet::Op {
            version,
            operation: operation_from_type(type_id),
            sub_packets,
        },
        index,
    )
}

fn parse_op(bytes: &[u8], start: usize, version: u64, type_id: u8) -> (Packet, usize) {
    if read_bit(bytes, start) {
        parse_count_op(bytes, start + 1, version, type_id)
    } else {
        parse_length_op(bytes, start + 1, version, type_id)
    }
}

fn parse_packet(bytes: &[u8], start: usize) -> (Packet, usize) {
    let mut index = start;

    let version = read_triplet(bytes, index);
    index += 3;

    let type_id = read_triplet(bytes, index);
    index += 3;

    if type_id == 4 {
        return parse_literal(bytes, index, version as u64);
    }

    parse_op(bytes, index, version as u64, type_id)
}

fn parse_hex(input: &str) -> Packet {
    let bytes = hex::decode(input).unwrap();

    let (packet, _) = parse_packet(&bytes, 0);

    packet
}

fn sum_version(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { version, .. } => *version,
        Packet::Op {
            version,
            sub_packets,
            ..
        } => *version + sub_packets.iter().map(sum_version).sum::<u64>(),
    }
}

fn calculate(packet: &Packet) -> i64 {
    match packet {
        Packet::Literal { value, .. } => *value,
        Packet::Op {
            operation,
            sub_packets,
            ..
        } => {
            let mut values = sub_packets.iter().map(calculate);

            match operation {
                Operation::Sum => values.sum(),
                Operation::Product => values.product(),
                Operation::Minimum => values.min().unwrap(),
                Operation::Maximum => values.max().unwrap(),
                Operation::GreaterThan => {
                    if values.next().unwrap() > values.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                Operation::LessThan => {
                    if values.next().unwrap() < values.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                Operation::EqualTo => {
                    if values.next().unwrap() == values.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
            }
        }
    }
}

pub fn part_a(file: &str) -> Solution {
    Solution::Integer(sum_version(&parse_hex(file)) as i64)
}

pub fn part_b(file: &str) -> Solution {
    Solution::Integer(calculate(&parse_hex(file)) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Packet::Literal {
                version: 6,
                value: 2021
            },
            parse_hex("D2FE28")
        );
        assert_eq!(16, sum_version(&parse_hex("8A004A801A8002F478")));
        assert_eq!(12, sum_version(&parse_hex("620080001611562C8802118E34")));
        assert_eq!(23, sum_version(&parse_hex("C0015000016115A2E0802F182340")));
        assert_eq!(
            31,
            sum_version(&parse_hex("A0016C880162017C3686B18A3D4780"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(3, calculate(&parse_hex("C200B40A82")));
        assert_eq!(54, calculate(&parse_hex("04005AC33890")));
        assert_eq!(7, calculate(&parse_hex("880086C3E88112")));
        assert_eq!(9, calculate(&parse_hex("CE00C43D881120")));
        assert_eq!(0, calculate(&parse_hex("F600BC2D8F")));
        assert_eq!(0, calculate(&parse_hex("9C005AC2F8F0")));
        assert_eq!(1, calculate(&parse_hex("9C0141080250320F1802104A08")));
    }
}
