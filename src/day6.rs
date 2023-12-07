#![allow(dead_code)]
use std::io::Read;

fn clean_string(s: String) -> String {
    s.trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn simulate(race: (usize, usize)) -> usize {
    let mut possible_times = 0;
    for time_held in 0..race.0 + 1 {
        let travel_distance = (race.0 - time_held) * time_held;
        if travel_distance > race.1 {
            possible_times += 1;
        }
    }
    possible_times
}

pub fn run() {
    let mut file = std::fs::File::open("day6/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let data: Vec<_> = contents
        .split('\n')
        .map(|x| clean_string(x.to_string()))
        .collect();

    let times: Vec<usize> = data[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let distances: Vec<usize> = data[1]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let combined: Vec<(usize, usize)> = times
        .iter()
        .enumerate()
        .map(|(i, _)| (times[i], distances[i]))
        .collect();

    println!(
        "{:?}",
        combined.iter().map(|x| simulate(*x)).product::<usize>()
    );
    
    let unfucked_race: (usize, usize) = (
        data[0].split(": ").nth(1).unwrap().split(' ').collect::<Vec<_>>().join("").parse().unwrap(),
        data[1].split(": ").nth(1).unwrap().split(' ').collect::<Vec<_>>().join("").parse().unwrap(),
    );

    println!("{:?}", simulate(unfucked_race));
}
