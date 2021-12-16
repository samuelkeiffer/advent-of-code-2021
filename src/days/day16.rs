use crate::*;
use bitvec::prelude::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let Input(input) = get_input::<Input>("day-16");
    let bits_input = parse_input(input);
    let (packet, _remainder) = parse_packet(&bits_input);
    let value = resolve_packet_value(&packet);
    dbg!(value);
}

fn part1() {
    let Input(input) = get_input::<Input>("day-16");
    let bits_input = parse_input(input);
    let (packet, _remainder) = parse_packet(&bits_input);
    fn version_sum(packet: &Packet) -> u64 {
        let mut sum = packet.version as u64;
        match &packet.contents {
            PacketContents::Literal(_) => {}
            PacketContents::Operator {
                packets,
                operator: _operator,
            } => {
                for packet in packets {
                    sum += version_sum(packet);
                }
            }
        }
        sum
    }
    let sum = version_sum(&packet);
    dbg!(sum);
}

fn resolve_packet_value(packet: &Packet) -> u64 {
    match &packet.contents {
        PacketContents::Literal(num) => *num,
        PacketContents::Operator { operator, packets } => match operator {
            Operator::Sum => packets
                .iter()
                .fold(0, |sum, packet| sum + resolve_packet_value(packet)),
            Operator::Product => packets
                .iter()
                .fold(1, |prod, packet| prod * resolve_packet_value(packet)),
            Operator::Minimum => packets.iter().fold(u64::MAX, |min, packet| {
                min.min(resolve_packet_value(packet))
            }),
            Operator::Maximum => packets.iter().fold(u64::MIN, |max, packet| {
                max.max(resolve_packet_value(packet))
            }),
            Operator::GreaterThan => {
                assert!(packets.len() == 2);
                if resolve_packet_value(&packets[0]) > resolve_packet_value(&packets[1]) {
                    1
                } else {
                    0
                }
            }
            Operator::LessThan => {
                assert!(packets.len() == 2);
                if resolve_packet_value(&packets[0]) < resolve_packet_value(&packets[1]) {
                    1
                } else {
                    0
                }
            }
            Operator::EqualTo => {
                assert!(packets.len() == 2);
                if resolve_packet_value(&packets[0]) == resolve_packet_value(&packets[1]) {
                    1
                } else {
                    0
                }
            }
        },
    }
}

fn parse_packet(input: &BitSlice) -> (Packet, BitVec) {
    let version = parse_header(&input[0..3]);
    let kind = parse_header(&input[3..6]);
    let (contents, remainder) = match kind {
        4 => {
            let (literal, remainder) = parse_literal(&input[6..]);
            (PacketContents::Literal(literal), remainder)
        }
        op_kind => {
            let (packets, remainder) = parse_operator(&input[6..]);
            let operator = get_op_kind(op_kind);
            (PacketContents::Operator { operator, packets }, remainder)
        }
    };
    (
        Packet {
            version,
            // kind,
            contents,
        },
        remainder,
    )
}

fn parse_operator(slice: &BitSlice) -> (Vec<Packet>, BitVec) {
    fn parse_length(slice: &BitSlice) -> usize {
        slice.iter().fold(0, |num, bit| {
            let mut num = num << 1;
            if *bit {
                num += 1;
            }
            num
        })
    }
    let length_type = slice[0];
    match length_type {
        false => {
            let length = parse_length(&slice[1..=15]);
            let mut remaining_slice = slice[16..(16 + length)].to_bitvec();
            let mut packets = Vec::new();
            while !remaining_slice.is_empty() {
                let (next_packet, new_remaining_slice) = parse_packet(&remaining_slice);
                packets.push(next_packet);
                remaining_slice = new_remaining_slice;
            }
            let remainder = slice[(16 + length)..].to_bitvec();
            (packets, remainder)
        }
        true => {
            let num_packets = parse_length(&slice[1..=11]);
            let mut remaining_slice = slice[12..].to_bitvec();
            let mut packets = Vec::new();
            for _ in 0..num_packets {
                let (next_packet, new_remaining_slice) = parse_packet(&remaining_slice);
                packets.push(next_packet);
                remaining_slice = new_remaining_slice;
            }
            (packets, remaining_slice)
        }
    }
}

// (Literal, remaining bits)
fn parse_literal(slice: &BitSlice) -> (u64, BitVec) {
    fn parse_chunks(slice: &BitSlice, number_bits: &mut BitVec) -> BitVec {
        number_bits.extend_from_bitslice(&slice[1..5]);
        if slice[0] {
            parse_chunks(&slice[5..], number_bits)
        } else {
            slice[5..].to_bitvec()
        }
    }
    let mut number_bits = bitvec![];
    let remainder = parse_chunks(slice, &mut number_bits);
    let literal = number_bits.iter().fold(0, |num, bit| {
        let mut num = num << 1;
        if *bit {
            num += 1;
        }
        num
    });
    (literal, remainder)
}

fn parse_header(slice: &BitSlice) -> u8 {
    assert!(slice.len() == 3);
    slice.iter().fold(0u8, |header, bit| {
        let mut header = header << 1;
        if *bit {
            header += 1;
        }
        header
    })
}

fn get_op_kind(id: u8) -> Operator {
    match id {
        0 => Operator::Sum,
        1 => Operator::Product,
        2 => Operator::Minimum,
        3 => Operator::Maximum,
        5 => Operator::GreaterThan,
        6 => Operator::LessThan,
        7 => Operator::EqualTo,
        _ => unimplemented!(),
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, Clone)]
enum PacketContents {
    Literal(u64),
    Operator {
        operator: Operator,
        packets: Vec<Packet>,
    },
}

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    // kind: u8,
    contents: PacketContents,
}

fn parse_input(input: Vec<String>) -> BitVec {
    let mut bv = bitvec![];
    for hex in input {
        match hex.as_str() {
            "0" => bv.extend_from_bitslice(bits![0, 0, 0, 0]),
            "1" => bv.extend_from_bitslice(bits![0, 0, 0, 1]),
            "2" => bv.extend_from_bitslice(bits![0, 0, 1, 0]),
            "3" => bv.extend_from_bitslice(bits![0, 0, 1, 1]),
            "4" => bv.extend_from_bitslice(bits![0, 1, 0, 0]),
            "5" => bv.extend_from_bitslice(bits![0, 1, 0, 1]),
            "6" => bv.extend_from_bitslice(bits![0, 1, 1, 0]),
            "7" => bv.extend_from_bitslice(bits![0, 1, 1, 1]),
            "8" => bv.extend_from_bitslice(bits![1, 0, 0, 0]),
            "9" => bv.extend_from_bitslice(bits![1, 0, 0, 1]),
            "A" => bv.extend_from_bitslice(bits![1, 0, 1, 0]),
            "B" => bv.extend_from_bitslice(bits![1, 0, 1, 1]),
            "C" => bv.extend_from_bitslice(bits![1, 1, 0, 0]),
            "D" => bv.extend_from_bitslice(bits![1, 1, 0, 1]),
            "E" => bv.extend_from_bitslice(bits![1, 1, 1, 0]),
            "F" => bv.extend_from_bitslice(bits![1, 1, 1, 1]),
            _ => unreachable!(),
        }
    }
    bv
}

#[derive(Clone, Deserialize)]
struct Input(Vec<String>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
