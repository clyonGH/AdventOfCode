use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug)]
enum PacketData {
    Integer(u8),
    List(Vec<PacketData>),
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(val), Self::Integer(other_val)) => val.cmp(other_val),
            (Self::List(pdata), Self::Integer(_)) => {
                let temp_vec: Vec<PacketData> = vec![other.clone()];
                pdata.cmp(&temp_vec)
            }
            (Self::Integer(_), Self::List(pdata)) => {
                let temp_vec: Vec<PacketData> = vec![self.clone()];
                temp_vec.cmp(pdata)
            }
            (Self::List(pdata), Self::List(other_pdata)) => pdata.cmp(other_pdata),
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut file = File::open("inputs/input_2022_13.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut all_pdata: Vec<PacketData> = data
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|line| get_packet_data(&line))
        .collect();

    // first part
    let mut i_sum: usize = 0;
    all_pdata.chunks(2).enumerate().for_each(|(index, chunks)| {
        if chunks[0].cmp(&chunks[1]) == Ordering::Less {
            i_sum += index + 1;
        }
    });

    println!("the sum of the indices of those pairs is: {i_sum}");

    // second part
    let dpacket1 = get_packet_data("[[2]]");
    all_pdata.push(dpacket1.clone());
    let dpacket2 = get_packet_data("[[6]]");
    all_pdata.push(dpacket2.clone());
    all_pdata.sort();

    let mut i_product: usize = 1;
    all_pdata.iter().enumerate().for_each(|(index, pdata)| {
        if pdata.cmp(&dpacket1) == Ordering::Equal || pdata.cmp(&dpacket2) == Ordering::Equal {
            i_product *= index + 1;
        }
    });

    println!("the decoder key for the distress signal is: {i_product}");
}

fn parse_data(data: &str) -> Vec<&str> {
    let mut range: u8 = 0;
    let mut all_commas: Vec<usize> = Vec::new();
    all_commas.push(0);
    data.chars().enumerate().for_each(|(index, c)| match c {
        '[' => {
            range += 1;
        }
        ']' => {
            range -= 1;
        }
        ',' => {
            if range == 1 {
                all_commas.push(index);
            }
        }
        _ => (),
    });
    all_commas.push(data.len() - 1);

    all_commas
        .windows(2)
        .map(|window| {
            let min: usize = window[0];
            let max: usize = window[1];
            &data[min + 1..max]
        })
        .collect()
}

fn get_packet_data(data: &str) -> PacketData {
    if let Ok(int) = data.parse::<u8>() {
        PacketData::Integer(int)
    } else {
        let parsed_data = parse_data(data);
        PacketData::List(
            parsed_data
                .iter()
                .filter(|&pdata| !pdata.is_empty())
                .map(|&pdata| get_packet_data(pdata))
                .collect(),
        )
    }
}
