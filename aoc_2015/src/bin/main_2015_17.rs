use std::fs::File;
use std::io::prelude::*;

const VOL_EGGNOG: u8 = 150;
const MIN_BUCKETS: u8 = 4;

fn main() {
    let mut file = File::open("inputs/input_2015_17.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut sorted_buckets: Vec<u8> = data
        .lines()
        .map(|bucket| bucket.parse::<u8>().unwrap())
        .collect();
    sorted_buckets.sort();

    // first part
    let nb_comb = fill_buckets(VOL_EGGNOG, sorted_buckets.clone());
    println!("there are {nb_comb} different combinations of containers");

    // second part
    let diff_ways = fill_4_buckets(VOL_EGGNOG, sorted_buckets.clone(), MIN_BUCKETS);
    println!("there are {diff_ways} different ways to fill 4 containers");
}

fn fill_buckets(volume: u8, mut buckets: Vec<u8>) -> u32 {
    if volume == 0 {
        return 1;
    }

    if buckets.is_empty() {
        return 0;
    }

    let first_bucket = buckets.pop().unwrap();
    if volume < first_bucket {
        fill_buckets(volume, buckets)
    } else {
        fill_buckets(volume, buckets.clone()) + fill_buckets(volume - first_bucket, buckets)
    }
}

fn fill_4_buckets(volume: u8, mut buckets: Vec<u8>, nb_buckets: u8) -> u32 {
    if volume == 0 {
        return 1;
    }

    if buckets.is_empty() || nb_buckets == 0 {
        return 0;
    }

    let first_bucket = buckets.pop().unwrap();
    if volume < first_bucket {
        fill_4_buckets(volume, buckets, nb_buckets)
    } else {
        fill_4_buckets(volume, buckets.clone(), nb_buckets)
            + fill_4_buckets(volume - first_bucket, buckets, nb_buckets - 1)
    }
}
