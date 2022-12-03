use md5;
use std::collections::HashMap;

const HEX_ARRAY: &[u8; 16] = b"0123456789abcdef";
const FINAL_KEY: usize = 64;
const MAX_DIFF: u64 = 1000;

fn main() {
    let input: &str = "qzyelonm";

    // first part
    let mut i: u64 = 0;
    let mut j: u64 = 0;
    let mut keys_found: Vec<u64> = Vec::new();
    let mut potential_keys: HashMap<u64, char> = HashMap::new();
    loop {
        let data_incr = format!("{input}{i}");
        let digest = md5::compute(data_incr);
        let digest_hex = format!("{:x}", digest);

        let char_option: Option<char> = has_three_letters_in_a_row(digest_hex.as_str());
        if let Some(char_found) = char_option {
            let chars_found: Vec<char> = has_five_letters_in_a_row(digest_hex.as_str());

            if !chars_found.is_empty() {
                // removing all potential keys
                potential_keys.retain(|&k, _| i - k <= MAX_DIFF);

                // checking if a hash is a key
                chars_found.iter().for_each(|c| {
                    potential_keys.retain(|&k, v| {
                        if c == v {
                            keys_found.push(k);
                        }

                        c != v
                    });
                });
            }

            if keys_found.len() >= FINAL_KEY {
                j += 1;

                // make sure there aren't validated keys in the following 1000 after the 64th was found
                // if there is, then the 64th key changes
                if j > MAX_DIFF {
                    keys_found.sort();

                    println!(
                        "the 64th one-time pad key was found at index: {:?}",
                        keys_found[FINAL_KEY - 1]
                    );
                    break;
                }
            }

            potential_keys.insert(i, char_found);
        }

        i += 1;
    }

    // second part
    i = 0;
    j = 0;
    keys_found.clear();
    potential_keys.clear();
    loop {
        let data_incr = format!("{input}{i}");
        let mut digest = md5::compute(data_incr);

        (0..2016).for_each(|_| {
            digest = md5::compute(bytes_to_hex(digest.0));
        });

        let digest_hex_str: String = format!("{:x}", digest);

        let char_option: Option<char> = has_three_letters_in_a_row(digest_hex_str.as_str());
        if let Some(char_found) = char_option {
            let chars_found: Vec<char> = has_five_letters_in_a_row(digest_hex_str.as_str());

            if !chars_found.is_empty() {
                // removing all potential keys
                potential_keys.retain(|&k, _| i - k <= MAX_DIFF);

                // checking if a hash is a key
                chars_found.iter().for_each(|c| {
                    potential_keys.retain(|&k, v| {
                        if c == v {
                            keys_found.push(k);
                        }

                        c != v
                    });
                });
            }

            if keys_found.len() >= FINAL_KEY {
                j += 1;

                // make sure there aren't validated keys in the following 1000 after the 64th was found
                // if there is, then the 64th key changes
                if j > MAX_DIFF {
                    keys_found.sort();

                    println!(
                        "the 64th one-time pad key was found at index: {:?}",
                        keys_found[FINAL_KEY - 1]
                    );
                    break;
                }
            }

            potential_keys.insert(i, char_found);
        }

        i += 1;
    }
}

fn bytes_to_hex(bytes: [u8; 16]) -> [u8; 32] {
    let mut hex: [u8; 32] = [0; 32];
    for i in 0..bytes.len() {
        let v: usize = bytes[i] as usize;
        hex[i * 2] = HEX_ARRAY[v >> 4];
        hex[i * 2 + 1] = HEX_ARRAY[v & 0x0F];
    }
    hex
}

fn has_three_letters_in_a_row(input_line: &str) -> Option<char> {
    input_line
        .chars()
        .collect::<Vec<char>>()
        .windows(3)
        .find_map(|window| {
            if window[0] == window[1] && window[1] == window[2] {
                Some(window[0])
            } else {
                None
            }
        })
}

fn has_five_letters_in_a_row(input_line: &str) -> Vec<char> {
    let mut chars_found: Vec<char> = Vec::new();
    input_line
        .chars()
        .collect::<Vec<char>>()
        .windows(5)
        .for_each(|window| {
            if window[0] == window[1]
                && window[1] == window[2]
                && window[2] == window[3]
                && window[3] == window[4]
            {
                chars_found.push(window[0]);
            }
        });

    chars_found
}
