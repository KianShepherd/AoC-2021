use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct Packet {
    packet_version: u64,
    packet_type: u64,
    packet_value: u64,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn sum_versions(&self) -> u64 {
        if self.packet_type == 4 {
            self.packet_version
        } else {
            let mut version_value = self.packet_version;
            for pack in &self.sub_packets {
                version_value += pack.sum_versions();
            }
            version_value
        }
    }

    fn get_value(&self) -> u64 {
        match self.packet_type {
            0 => {
                let mut value = 0;
                for sub_packet in &self.sub_packets {
                    value += sub_packet.get_value();
                }
                value
            }
            1 => {
                let mut value = 1;
                for sub_packet in &self.sub_packets {
                    value *= sub_packet.get_value();
                }
                value
            }
            2 => {
                let mut minimum = std::u64::MAX;
                for sub_packet in &self.sub_packets {
                    let sub_packet_value = sub_packet.get_value();
                    if sub_packet_value < minimum {
                        minimum = sub_packet_value;
                    }
                }
                minimum
            }
            3 => {
                let mut maximum = 0;
                for sub_packet in &self.sub_packets {
                    let sub_packet_value = sub_packet.get_value();
                    if sub_packet_value > maximum {
                        maximum = sub_packet_value;
                    }
                }
                maximum
            }
            4 => self.packet_value,
            5 => {
                if self.sub_packets[0].get_value() > self.sub_packets[1].get_value() {
                    1
                } else {
                    0
                }
            }
            6 => {
                if self.sub_packets[0].get_value() < self.sub_packets[1].get_value() {
                    1
                } else {
                    0
                }
            }
            7 => {
                if self.sub_packets[0].get_value() == self.sub_packets[1].get_value() {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}

fn binary_to_u64(binary: Vec<char>) -> u64 {
    let mut bin = binary.clone();
    bin.reverse();
    let mut sum = 0;
    for (index, chr) in bin.into_iter().enumerate() {
        if chr == '1' {
            sum += 2_u64.pow(index as u32);
        }
    }

    sum
}

fn get_binary_from_hex(hex_val: char) -> String {
    match hex_val {
        '0' => "0000".to_string(),
        '1' => "0001".to_string(),
        '2' => "0010".to_string(),
        '3' => "0011".to_string(),
        '4' => "0100".to_string(),
        '5' => "0101".to_string(),
        '6' => "0110".to_string(),
        '7' => "0111".to_string(),
        '8' => "1000".to_string(),
        '9' => "1001".to_string(),
        'A' => "1010".to_string(),
        'B' => "1011".to_string(),
        'C' => "1100".to_string(),
        'D' => "1101".to_string(),
        'E' => "1110".to_string(),
        'F' => "1111".to_string(),
        _ => "".to_string(),
    }
}

fn parse_value(binary: &Vec<char>, version: u64, type_num: u64) -> (Packet, usize) {
    let mut binary_iter = binary.iter();
    let mut values_binary = vec![];
    let mut packet_length = 6;

    loop {
        packet_length += 5;
        let last_set = *binary_iter.next().unwrap() == '0';
        values_binary.push(*binary_iter.next().unwrap());
        values_binary.push(*binary_iter.next().unwrap());
        values_binary.push(*binary_iter.next().unwrap());
        values_binary.push(*binary_iter.next().unwrap());

        if last_set {
            break;
        }
    }
    let packet_value = binary_to_u64(values_binary);

    (
        Packet {
            packet_version: version,
            packet_type: type_num,
            packet_value,
            sub_packets: vec![],
        },
        packet_length,
    )
}

fn parse_packets(binary: Vec<char>) -> (Packet, usize) {
    let mut binary_iter = binary.iter();
    let mut packet_length = 0;

    let version_binary = vec![
        *binary_iter.next().unwrap(),
        *binary_iter.next().unwrap(),
        *binary_iter.next().unwrap(),
    ];
    let packet_version = binary_to_u64(version_binary);
    let type_binary = vec![
        *binary_iter.next().unwrap(),
        *binary_iter.next().unwrap(),
        *binary_iter.next().unwrap(),
    ];
    let packet_type = binary_to_u64(type_binary);

    packet_length += 6;

    if packet_type == 4 {
        let packet_value = parse_value(
            &(binary_iter.clone().copied().collect::<Vec<char>>()),
            packet_version,
            packet_type,
        );
        for _ in 0..packet_value.1 {
            binary_iter.next();
            packet_length += 1;
        }

        packet_value
    } else {
        packet_length += 1;
        if *binary_iter.next().unwrap() == '0' {
            let mut num_bits_binary = vec![];
            packet_length += 15;
            for _ in 0..15 {
                num_bits_binary.push(*binary_iter.next().unwrap());
            }
            let num_bits = binary_to_u64(num_bits_binary);
            let mut sub_packets = vec![];
            let mut bits_count = 0;
            loop {
                let parsed_packet =
                    parse_packets(binary_iter.clone().copied().collect::<Vec<char>>());
                sub_packets.push(parsed_packet.0);
                for _ in 0..parsed_packet.1 {
                    binary_iter.next();
                    packet_length += 1;
                    bits_count += 1;
                }
                if bits_count >= num_bits {
                    break;
                }
            }

            (
                Packet {
                    packet_version,
                    packet_type,
                    packet_value: 0,
                    sub_packets,
                },
                packet_length,
            )
        } else {
            let mut num_packets_binary = vec![];
            packet_length += 11;
            for _ in 0..11 {
                num_packets_binary.push(*binary_iter.next().unwrap());
            }
            let num_packets = binary_to_u64(num_packets_binary);
            let mut sub_packets = vec![];
            for _ in 0..num_packets {
                let parsed_packet =
                    parse_packets(binary_iter.clone().copied().collect::<Vec<char>>());
                sub_packets.push(parsed_packet.0);
                for _ in 0..parsed_packet.1 {
                    binary_iter.next();
                    packet_length += 1;
                }
            }

            (
                Packet {
                    packet_version,
                    packet_type,
                    packet_value: 0,
                    sub_packets,
                },
                packet_length,
            )
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let start = Instant::now();
    let v = contents.trim().chars();
    let mut binary_str = "".to_string();
    for chr in v {
        //println!("{:?}", chr);
        binary_str.push_str(&get_binary_from_hex(chr));
    }
    let binary = binary_str.chars().collect::<Vec<char>>();
    let packet = parse_packets(binary);
    //println!("{:?}", packet);
    println!(
        "{} : {} : took {:?}",
        packet.0.sum_versions(),
        packet.0.get_value(),
        start.elapsed()
    );
}
