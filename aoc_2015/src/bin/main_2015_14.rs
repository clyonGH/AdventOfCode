use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_14.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut all_data: Vec<(&str, u16, u16, u16, u16)> = Vec::new(); // (name of reeindeer, speed, speed duration, rest duration, distance)
    data.lines().for_each(|line| {
        let split_data: Vec<&str> = line.split(' ').collect();
        all_data.push((
            split_data[0],
            split_data[3].parse::<u16>().unwrap(),
            split_data[6].parse::<u16>().unwrap(),
            split_data[13].parse::<u16>().unwrap(),
            0,
        ));
    });

    const RACE_LENGTH: u16 = 2503;
    const NB_REINDEERS: usize = 9;

    // first part
    println!(
        "[1st scoring] the winning reindeer traveled {:?} km",
        get_lead(&all_data, RACE_LENGTH).iter().max().unwrap()
    );

    // second part
    let mut all_points: [u16; NB_REINDEERS] = [0; NB_REINDEERS];
    for current_time in 1..RACE_LENGTH {
        let new_lead = get_lead(&all_data, current_time)
            .iter()
            .cloned()
            .collect::<Vec<u16>>();

        let &max_score = new_lead.iter().max().unwrap();

        new_lead.iter().enumerate().for_each(|(reindeer, score)| {
            if score == &max_score {
                all_points[reindeer] += 1;
            }
        });
    }

    println!(
        "[2nd scoring] the winning reindeer has {:?} points",
        all_points.iter().max().unwrap()
    );
}

fn get_lead(all_data: &Vec<(&str, u16, u16, u16, u16)>, race_length: u16) -> Vec<u16> {
    all_data
        .iter()
        .map(|reindeer_data| {
            let speed = reindeer_data.1;
            let speed_duration = reindeer_data.2;
            let rest_duration = reindeer_data.3;
            let full_cycle = race_length / (speed_duration + rest_duration);
            let remainder_cycle = race_length % (speed_duration + rest_duration);

            let reindeer_dist = if remainder_cycle > speed_duration {
                (full_cycle + 1) * speed * speed_duration
            } else {
                speed * (full_cycle * speed_duration + remainder_cycle)
            };
            reindeer_dist
        })
        .collect::<Vec<u16>>()
}
