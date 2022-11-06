use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2016_07.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    // first part
    let ips_with_tls_support: usize = data
        .lines()
        .filter(|ip| {
            let split_data: Vec<&str> = ip.split(|pat| pat == '[' || pat == ']').collect();
            let mut all_supernets: Vec<&str> = Vec::new();
            let mut all_hypernets: Vec<&str> = Vec::new();
            let mut tls_support: bool = false;
            split_data.iter().enumerate().for_each(|(index, &seq)| {
                if index % 2 == 0 {
                    all_supernets.push(seq);
                } else {
                    all_hypernets.push(seq);
                }
            });

            all_supernets.iter().for_each(|&seq| {
                if has_abba(seq) {
                    tls_support = true;
                }
            });

            all_hypernets.iter().for_each(|&seq| {
                if has_abba(seq) {
                    tls_support = false;
                }
            });

            tls_support
        })
        .count();

    println!("{:?} IPs support TLS", ips_with_tls_support);

    // second part
    let ips_with_ssl_support: usize = data
        .lines()
        .filter(|ip| {
            let split_data: Vec<&str> = ip.split(|pat| pat == '[' || pat == ']').collect();
            let mut all_supernets: Vec<&str> = Vec::new();
            let mut all_hypernets: Vec<&str> = Vec::new();
            let mut all_abas: Vec<String> = Vec::new();
            let mut all_babs: Vec<String> = Vec::new();
            split_data.iter().enumerate().for_each(|(index, &seq)| {
                if index % 2 == 0 {
                    all_supernets.push(seq);
                } else {
                    all_hypernets.push(seq);
                }
            });

            all_supernets.iter().for_each(|&seq| {
                if has_aba(seq).is_some() {
                    has_aba(seq)
                        .unwrap()
                        .iter()
                        .for_each(|aba| all_abas.push(aba.clone()));
                }
            });

            all_hypernets.iter().for_each(|&seq| {
                if has_aba(seq).is_some() {
                    has_aba(seq)
                        .unwrap()
                        .iter()
                        .for_each(|bab| all_babs.push(bab.clone()));
                }
            });

            all_babs.iter().any(|bab| {
                all_abas
                    .iter()
                    .any(|aba| is_bab_to_aba(aba.clone(), bab.clone()))
            })
        })
        .count();

    println!("{:?} IPs support SSL", ips_with_ssl_support);
}

fn has_abba(input_line: &str) -> bool {
    input_line
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .any(|window| window[0] == window[3] && window[1] == window[2] && window[0] != window[1])
}

fn has_aba(input_line: &str) -> Option<Vec<String>> {
    let mut pattern = String::from("");
    let all_aba: Vec<String> = input_line
        .chars()
        .collect::<Vec<char>>()
        .windows(3)
        .filter_map(|window| {
            if window[0] == window[2] && window[0] != window[1] {
                pattern.clear();
                pattern.push(window[0]);
                pattern.push(window[1]);
                pattern.push(window[2]);
                Some(pattern.clone())
            } else {
                None
            }
        })
        .collect();

    if all_aba.is_empty() {
        None
    } else {
        Some(all_aba)
    }
}

fn is_bab_to_aba(aba: String, bab: String) -> bool {
    (aba.chars().nth(0) == bab.chars().nth(1)) && (aba.chars().nth(1) == bab.chars().nth(0))
}
