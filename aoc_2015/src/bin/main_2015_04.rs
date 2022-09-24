use md5;
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_04.txt").expect("Error: File not found");

    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut i = 1;

    // first part
    loop {
        let data_incr = format!("{data}{i}");
        let digest = md5::compute(data_incr);
        let digest_hex = format!("{:x}", digest);

        if digest_hex.starts_with("00000") {
            println!("hash found: {}", digest_hex);
            println!("the answer is: {}\n", i);
            break;
        }

        i += 1;
    }

    // second part
    (1..10000000).into_par_iter().for_each(|i| {
        let data_incr = format!("{data}{i}");
        let digest = md5::compute(data_incr);
        let digest_hex = format!("{:x}", digest);

        if digest_hex.starts_with("000000") {
            println!("hash found: {}", digest_hex);
            println!("the answer is: {}\n", i);
        }
    });
}
