use md5;
use std::fs::File;
use std::io::prelude::*;

const PWD_SIZE: usize = 8;

fn main() {
    let mut file = File::open("inputs/input_2016_05.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut i: u32 = 1;
    let mut j: usize = 0;
    let mut password: Vec<char> = Vec::new();

    // first part
    loop {
        let data_incr = format!("{data}{i}");
        let digest = md5::compute(data_incr);
        let digest_hex = format!("{:x}", digest);

        if digest_hex.starts_with("00000") {
            println!("hash found: {}", digest_hex);
            password.push(digest_hex.as_bytes()[5] as char);
            j += 1;
            if j == PWD_SIZE {
                println!("the password is: {:?}", password.iter().collect::<String>());
                break;
            }
        }
        i += 1;
    }

    // second part
    let mut new_password: [char; PWD_SIZE] = [' '; PWD_SIZE];
    i = 0;
    j = 0;
    loop {
        let data_incr = format!("{data}{i}");
        let digest = md5::compute(data_incr);
        let digest_hex = format!("{:x}", digest);

        if digest_hex.starts_with("00000") {
            println!("hash found: {}", digest_hex);
            let position = digest_hex.as_bytes()[5] as usize - '0' as usize;
            if position < PWD_SIZE && new_password[position] == ' ' {
                new_password[position] = digest_hex.as_bytes()[6] as char;
                println!("{position}\t{:?}", new_password);
                j += 1;
                if j == PWD_SIZE {
                    println!(
                        "the new password is: {:?}",
                        new_password.iter().collect::<String>()
                    );
                    break;
                }
            }
        }
        i += 1;
    }
}
