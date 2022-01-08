use std::str::Chars;

use crate::day16::PacketType::*;
use crate::util::load;

struct BitStream<'a> {
    hex: Chars<'a>,
    hex_offset: usize,
    nibble: u8,
    bits_left: usize,
}

impl<'a> BitStream<'a> {
    fn new(hex: &'a str) -> BitStream<'a> {
        BitStream { hex: hex.chars(), hex_offset: 0, nibble: 0, bits_left: 0 }
    }

    fn advance_nibble(&mut self) {
        self.nibble = match self.hex.next() {
            Some(c) if c >= '0' && c <= '9' => c as u8 - '0' as u8,
            Some(c) if c >= 'A' && c <= 'F' => c as u8 - 'A' as u8 + 10,
            _ => unreachable!("bit stream corrupted"),
        };
        self.hex_offset += 1;
        self.bits_left = 4;
    }

    fn read_int(&mut self, bits: usize) -> u64 {
        let mut left = bits;
        let mut result = 0;
        while left > 0 {
            if self.bits_left >= left {
                self.bits_left -= left;
                result <<= left;
                result |= ((self.nibble >> self.bits_left) & ((1 << left) - 1)) as u64;
                left = 0;
            } else {
                result <<= self.bits_left;
                result |= (self.nibble & ((1 << self.bits_left) - 1)) as u64;
                left -= self.bits_left;
                self.advance_nibble();
            }
        }
        result
    }

    fn read_bit(&mut self) -> bool {
        if self.bits_left == 0 {
            self.advance_nibble();
        }
        self.bits_left -= 1;
        let bit = (self.nibble >> self.bits_left) & 1 == 1;
        bit
    }

    fn bit_offset(&self) -> usize {
        self.hex_offset * 4 - self.bits_left
    }
}

#[derive(Debug, PartialEq)]
enum PacketType {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

impl PacketType {
    fn new(ptype: u8, data: Vec<Packet>) -> PacketType {
        match ptype {
            0 => PacketType::Sum(data),
            1 => PacketType::Product(data),
            2 => PacketType::Minimum(data),
            3 => PacketType::Maximum(data),
            5 => PacketType::GreaterThan(data),
            6 => PacketType::LessThan(data),
            7 => PacketType::EqualTo(data),
            _ => unreachable!("bit stream corrupted"),
        }
    }

    fn eval(&self) -> u64 {
        fn eval_vec(data: &Vec<Packet>) -> impl Iterator<Item=u64> + '_ {
            data.iter().map(|p| p.body.eval())
        }

        fn eval_cond<F>(data: &Vec<Packet>, pred: F) -> u64
            where F: Fn(u64, u64) -> bool
        {
            let a = data[0].body.eval();
            let b = data[1].body.eval();
            if pred(a, b) { 1 } else { 0 }
        }

        match self {
            Sum(data) => eval_vec(data).sum(),
            Product(data) => eval_vec(data).product(),
            Minimum(data) => eval_vec(data).min().unwrap(),
            Maximum(data) => eval_vec(data).max().unwrap(),
            Literal(v) => *v as u64,
            GreaterThan(data) => eval_cond(data, |a, b| a > b),
            LessThan(data) => eval_cond(data, |a, b| a < b),
            EqualTo(data) => eval_cond(data, |a, b| a == b),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    body: PacketType,
}

impl<'a> From<BitStream<'a>> for Packet {
    fn from(mut bs: BitStream) -> Self {
        fn read_literal(bs: &mut BitStream) -> u64 {
            let mut result = 0;
            loop {
                let more = bs.read_bit();
                result <<= 4;
                result |= bs.read_int(4);
                if !more {
                    break result;
                }
            }
        }

        fn read_op_bits(bs: &mut BitStream, bits: usize) -> Vec<Packet> {
            let dst = bs.bit_offset() + bits;
            let mut data = Vec::new();
            while bs.bit_offset() < dst {
                let packet = read_packet(bs);
                data.push(packet)
            }
            data
        }

        fn read_op_packets(bs: &mut BitStream, packets: usize) -> Vec<Packet> {
            (0..packets)
                .map(|_i| read_packet(bs))
                .collect()
        }

        fn read_operator(bs: &mut BitStream) -> Vec<Packet> {
            let ltype = bs.read_bit();
            if ltype {
                let len = bs.read_int(11) as usize;
                read_op_packets(bs, len)
            } else {
                let len = bs.read_int(15) as usize;
                read_op_bits(bs, len)
            }
        }

        fn read_packet(bs: &mut BitStream) -> Packet {
            let version = bs.read_int(3) as u8;
            match bs.read_int(3) {
                4 => Packet { version, body: Literal(read_literal(bs)) },
                ptype => Packet {
                    version,
                    body: PacketType::new(ptype as u8, read_operator(bs)),
                },
            }
        }

        read_packet(&mut bs)
    }
}


fn input() -> Packet {
    let mut lines: Vec<String> = load("data/day16.txt");
    Packet::from(BitStream::new(&lines.remove(0)))
}

fn version_sum(packet: &Packet) -> u32 {
    match &packet.body {
        Literal(_) => packet.version as u32,
        Sum(data) | Product(data) |
        Minimum(data) | Maximum(data) |
        GreaterThan(data) | LessThan(data) |
        EqualTo(data)
        => (packet.version as u32)
            + data.iter().map(|p| version_sum(p)).sum::<u32>()
    }
}

fn part1(packet: Packet) -> u32 {
    version_sum(&packet)
}


fn part2(packet: Packet) -> u64 {
    packet.body.eval()
}

#[cfg(test)]
mod tests {
    use crate::day16::{BitStream, input, Packet, PacketType, part1, part2, version_sum};

    #[test]
    fn test_bitstream_read_int() {
        let mut bs = BitStream::new("ABC");
        assert_eq!(bs.read_int(3), 5);
        assert_eq!(bs.read_int(5), 11);
        assert_eq!(bs.read_int(4), 12)
    }

    #[test]
    fn test_packet_from_bitstream_literal() {
        let bs = BitStream::new("D2FE28");
        assert_eq!(Packet::from(bs), Packet { version: 6, body: PacketType::Literal(2021) })
    }

    #[test]
    fn test_packet_from_bitstream_bit_len() {
        let bs = BitStream::new("38006F45291200");
        assert_eq!(Packet::from(bs), Packet {
            version: 1,
            body: PacketType::LessThan(vec![
                Packet { version: 6, body: PacketType::Literal(10) },
                Packet { version: 2, body: PacketType::Literal(20) },
            ]),
        })
    }

    #[test]
    fn test_packet_from_bitstream_packet_num() {
        let bs = BitStream::new("EE00D40C823060");
        assert_eq!(Packet::from(bs), Packet {
            version: 7,
            body: PacketType::Maximum(vec![
                Packet { version: 2, body: PacketType::Literal(1) },
                Packet { version: 4, body: PacketType::Literal(2) },
                Packet { version: 1, body: PacketType::Literal(3) },
            ]),
        })
    }

    #[test]
    fn test_version_sum() {
        let bs = BitStream::new("8A004A801A8002F478");
        assert_eq!(version_sum(&Packet::from(bs)), 16);
        let bs = BitStream::new("620080001611562C8802118E34");
        assert_eq!(version_sum(&Packet::from(bs)), 12);
        let bs = BitStream::new("C0015000016115A2E0802F182340");
        assert_eq!(version_sum(&Packet::from(bs)), 23);
        let bs = BitStream::new("A0016C880162017C3686B18A3D4780");
        assert_eq!(version_sum(&Packet::from(bs)), 31);
    }

    #[test]
    fn test_eval() {
        let p = Packet::from(BitStream::new("C200B40A82"));
        assert_eq!(p.body.eval(), 3);
        let p = Packet::from(BitStream::new("04005AC33890"));
        assert_eq!(p.body.eval(), 54);
        let p = Packet::from(BitStream::new("880086C3E88112"));
        assert_eq!(p.body.eval(), 7);
        let p = Packet::from(BitStream::new("CE00C43D881120"));
        assert_eq!(p.body.eval(), 9);
        let p = Packet::from(BitStream::new("D8005AC2A8F0"));
        assert_eq!(p.body.eval(), 1);
        let p = Packet::from(BitStream::new("F600BC2D8F"));
        assert_eq!(p.body.eval(), 0);
        let p = Packet::from(BitStream::new("9C005AC2F8F0"));
        assert_eq!(p.body.eval(), 0);
        let p = Packet::from(BitStream::new("9C0141080250320F1802104A08"));
        assert_eq!(p.body.eval(), 1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 879);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 539051801941);
    }
}
