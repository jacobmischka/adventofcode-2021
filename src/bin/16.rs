use adventofcode_2021::get_input;

use std::{
    fmt::Write,
    io::{self, Cursor, Read},
};

fn main() {
    let input = get_input().unwrap();
    let mut binary = String::new();
    for num in input.trim().chars().map(|c| c.to_digit(16).unwrap()) {
        write!(binary, "{:04b}", num).unwrap();
    }

    let bits = to_bitvec(&binary);

    let mut cursor = Cursor::new(bits.as_slice());

    let packet = Packet::read(&mut cursor).unwrap();

    let mut v = 0;
    packet.walk_subpackets(&mut |p: &Packet| {
        v += p.version;
    });
    println!("Part 1: {}", v);
    println!("Part 2: {}", packet.value());
}

fn to_decimal(bits: &[u8]) -> u64 {
    let mut d = 0;

    for (i, b) in bits.iter().rev().enumerate() {
        d += *b as u64 * 2u64.pow(i as u32)
    }

    d
}

fn to_bitvec(s: &str) -> Vec<u8> {
    s.chars()
        .map(|c| match c {
            '1' => 1,
            '0' => 0,
            _ => panic!(),
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Packet {
    version: u64,
    packet_type: u64,
    contents: PacketContents,
}

#[derive(Debug, Clone)]
enum PacketContents {
    Literal(u64),
    SubPackets(Vec<Packet>),
}

impl Packet {
    fn read(cursor: &mut Cursor<&[u8]>) -> Result<Self, io::Error> {
        let mut one = [0u8; 1];
        let mut three = [0u8; 3];
        let mut five = [0u8; 5];
        let mut fifteen = [0u8; 15];
        let mut eleven = [0u8; 11];

        cursor.read(&mut three)?;
        let version = to_decimal(&three);
        cursor.read(&mut three)?;
        let packet_type = to_decimal(&three);

        match packet_type {
            4 => {
                let mut bits = Vec::new();
                loop {
                    cursor.read(&mut five)?;
                    for i in 1..5 {
                        bits.push(five[i]);
                    }

                    if five[0] == 0 {
                        break;
                    }
                }

                let num = to_decimal(&bits);

                Ok(Packet {
                    version,
                    packet_type,
                    contents: PacketContents::Literal(num),
                })
            }
            _ => {
                cursor.read(&mut one)?;
                let length_type = to_decimal(&one);
                let mut sub_packets = Vec::new();

                match length_type {
                    0 => {
                        cursor.read(&mut fifteen)?;
                        let total_len = to_decimal(&fifteen);
                        let start = cursor.position();
                        while cursor.position() < start + total_len as u64 {
                            sub_packets.push(Packet::read(cursor)?);
                        }
                    }
                    1 => {
                        cursor.read(&mut eleven)?;
                        let num_packets = to_decimal(&eleven);
                        for _ in 0..num_packets {
                            sub_packets.push(Packet::read(cursor)?);
                        }
                    }
                    _ => panic!(),
                }

                Ok(Packet {
                    version,
                    packet_type,
                    contents: PacketContents::SubPackets(sub_packets),
                })
            }
        }
    }

    fn walk_subpackets<F>(&self, func: &mut F)
    where
        F: FnMut(&Packet),
    {
        (*func)(self);

        match &self.contents {
            PacketContents::Literal(_) => {}
            PacketContents::SubPackets(packets) => {
                for packet in packets {
                    packet.walk_subpackets(func);
                }
            }
        }
    }

    fn value(&self) -> u64 {
        match &self.contents {
            PacketContents::Literal(val) => *val,
            PacketContents::SubPackets(sub_packets) => {
                let mut sub_values = sub_packets.iter().map(|p| p.value());
                match self.packet_type {
                    0 => sub_values.sum::<u64>(),
                    1 => sub_values.fold(1u64, |acc, p| acc * p),
                    2 => sub_values.min().unwrap(),
                    3 => sub_values.max().unwrap(),
                    5 => {
                        if sub_values.next().unwrap() > sub_values.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if sub_values.next().unwrap() < sub_values.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if sub_values.next().unwrap() == sub_values.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!(),
                }
            }
        }
    }
}
