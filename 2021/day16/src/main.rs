use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum PacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl TryFrom<u8> for PacketType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PacketType::Sum),
            1 => Ok(PacketType::Product),
            2 => Ok(PacketType::Minimum),
            3 => Ok(PacketType::Maximum),
            4 => Ok(PacketType::Literal),
            5 => Ok(PacketType::GreaterThan),
            6 => Ok(PacketType::LessThan),
            7 => Ok(PacketType::EqualTo),
            _ => Err(format!("Invalid packet type: {}", value)),
        }
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: PacketType,
    value: Option<u64>,
    sub_packets: Vec<Packet>,
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s).map_err(|e| e.to_string())?;
        let mut bit_offset = 0;
        Packet::decode(&bytes[..], &mut bit_offset)
    }
}
impl Packet {
    fn decode(bytes: &[u8], bit_offset: &mut usize) -> Result<Self, String> {
        // Get version and type_id
        let version = extract_field(bytes, bit_offset, 3) as u8;
        let type_id = PacketType::try_from(extract_field(bytes, bit_offset, 3) as u8)?;

        match type_id {
            PacketType::Literal => {
                // Literal value packet
                let value = parse_literal_value(bytes, bit_offset)?;
                Ok(Packet {
                    version,
                    type_id,
                    value: Some(value),
                    sub_packets: Vec::new(),
                })
            }
            _ => {
                let length_type_id = extract_field(bytes, bit_offset, 1);
                if length_type_id == 0 {
                    let total_length = extract_field(bytes, bit_offset, 15) as usize;
                    let target_offset = *bit_offset + total_length;
                    let mut sub_packets = Vec::new();
                    while *bit_offset < target_offset {
                        let sub_packet = Packet::decode(bytes, bit_offset)?;
                        sub_packets.push(sub_packet);
                    }
                    Ok(Packet {
                        version,
                        type_id,
                        value: None,
                        sub_packets,
                    })
                } else {
                    let num_sub_packets = extract_field(bytes, bit_offset, 11) as usize;
                    let mut sub_packets = Vec::new();
                    for _ in 0..num_sub_packets {
                        let sub_packet = Packet::decode(bytes, bit_offset)?;
                        sub_packets.push(sub_packet);
                    }
                    Ok(Packet {
                        version,
                        type_id,
                        value: None,
                        sub_packets,
                    })
                }
            }
        }
    }
}

fn parse_literal_value(bytes: &[u8], bit_offset: &mut usize) -> Result<u64, String> {
    let mut full_value = 0u64;
    loop {
        let prefix = extract_field(bytes, bit_offset, 1);
        let val = extract_field(bytes, bit_offset, 4);
        full_value = (full_value << 4) | val;
        if prefix == 0 {
            break;
        }
    }
    Ok(full_value)
}

fn extract_field(bytes: &[u8], bit_offset: &mut usize, length: usize) -> u64 {
    let mut value = 0u64;
    for _ in 0..length {
        let byte_offset = *bit_offset / 8;
        let bit_in_byte = *bit_offset % 8;
        value = (value << 1) | (((bytes[byte_offset] >> (7 - bit_in_byte)) & 1) as u64);
        *bit_offset += 1;
    }
    value
}

fn sum_versions(packet: &Packet, version_sum: &mut u64) {
    *version_sum += packet.version as u64;
    for sub_packet in &packet.sub_packets {
        sum_versions(sub_packet, version_sum);
    }
}

fn evaluate_expression(packet: &Packet) -> u64 {
    match packet.type_id {
        PacketType::Literal => packet.value.unwrap(),
        PacketType::Sum => packet.sub_packets.iter().map(evaluate_expression).sum(),
        PacketType::Product => packet.sub_packets.iter().map(evaluate_expression).product(),
        PacketType::Minimum => packet
            .sub_packets
            .iter()
            .map(evaluate_expression)
            .min()
            .unwrap(),
        PacketType::Maximum => packet
            .sub_packets
            .iter()
            .map(evaluate_expression)
            .max()
            .unwrap(),
        PacketType::GreaterThan => {
            let a = evaluate_expression(&packet.sub_packets[0]);
            let b = evaluate_expression(&packet.sub_packets[1]);
            if a > b { 1 } else { 0 }
        }
        PacketType::LessThan => {
            let a = evaluate_expression(&packet.sub_packets[0]);
            let b = evaluate_expression(&packet.sub_packets[1]);
            if a < b { 1 } else { 0 }
        }
        PacketType::EqualTo => {
            let a = evaluate_expression(&packet.sub_packets[0]);
            let b = evaluate_expression(&packet.sub_packets[1]);
            if a == b { 1 } else { 0 }
        }
    }
}

fn solve_p1(input: &str) -> u64 {
    let packet: Packet = input.parse().unwrap();
    let mut version_sum = 0;
    sum_versions(&packet, &mut version_sum);
    version_sum
}

fn solve_p2(input: &str) -> u64 {
    let packet: Packet = input.parse().unwrap();
    evaluate_expression(&packet)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let start = std::time::Instant::now();
    let answer = solve_p1(&input);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&input);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        assert_eq!(solve_p1("8A004A801A8002F478"), 16);
        assert_eq!(solve_p1("620080001611562C8802118E34"), 12);
        assert_eq!(solve_p1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(solve_p1("A0016C880162017C3686B18A3D4780"), 31);

        assert_eq!(solve_p2("C200B40A82"), 3);
        assert_eq!(solve_p2("04005AC33890"), 54);
        assert_eq!(solve_p2("880086C3E88112"), 7);
        assert_eq!(solve_p2("CE00C43D881120"), 9);
        assert_eq!(solve_p2("D8005AC2A8F0"), 1);
        assert_eq!(solve_p2("F600BC2D8F"), 0);
        assert_eq!(solve_p2("9C005AC2F8F0"), 0);
        assert_eq!(solve_p2("9C0141080250320F1802104A08"), 1);
    }

    #[test]
    fn test_parse_literal_packet() {
        let packet = "D2FE28".parse::<Packet>().unwrap();
        assert_eq!(packet.version, 6);
        assert_eq!(packet.type_id, PacketType::Literal);
        assert_eq!(packet.value.unwrap(), 2021);
    }

    #[test]
    fn test_extract_field() {
        let hex_string = "D2FE28";
        let bytes = hex::decode(hex_string).unwrap();
        let mut bit_offset = 0;

        let version = extract_field(&bytes, &mut bit_offset, 3);
        assert_eq!(version, 6);

        let type_id = extract_field(&bytes, &mut bit_offset, 3);
        assert_eq!(type_id, 4);

        let value = parse_literal_value(&bytes, &mut bit_offset).unwrap();
        assert_eq!(value, 2021);
    }

    #[test]
    fn test_parse_operator_packet() {
        let packet = "38006F45291200".parse::<Packet>().unwrap();
        assert_eq!(packet.version, 1);
        assert_eq!(packet.type_id, PacketType::LessThan);
        assert_eq!(packet.sub_packets.len(), 2);
        assert_eq!(packet.sub_packets[0].value.unwrap(), 10);
        assert_eq!(packet.sub_packets[1].value.unwrap(), 20);
    }

    #[test]
    fn test_parse_operator_packet_with_num_subpackets() {
        let packet = "EE00D40C823060".parse::<Packet>().unwrap();
        assert_eq!(packet.version, 7);
        assert_eq!(packet.type_id, PacketType::Maximum);
        assert_eq!(packet.sub_packets.len(), 3);
        assert_eq!(packet.sub_packets[0].value.unwrap(), 1);
        assert_eq!(packet.sub_packets[1].value.unwrap(), 2);
        assert_eq!(packet.sub_packets[2].value.unwrap(), 3);
    }
}
