#![allow(dead_code)]
use std::io::Read;

fn start_scratchcard_game(cards: Vec<usize>) -> usize {
    let mut all_cards: Vec<(usize, usize)> = cards.iter().map(|x| (1_usize, x.clone())).collect();

    for (i, winning) in cards.iter().enumerate() {
        for j in i+1..i+1+winning {
            if j >= cards.len() {continue}
            all_cards[j].0 += all_cards[i].0;
        }
    }
    let mut total = 0;
    for (copies, _) in all_cards {
        total += copies;
    }
    total
}

pub fn run() {
    let mut file = std::fs::File::open("day4/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut total_score = 0;
    let mut cards: Vec<usize> = Vec::new();

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }
        let csplit: Vec<&str> = line.split(':').collect();
        let data = csplit[1].trim();
        let nsplit: Vec<String> = data
            .split('|')
            .map(|x| {
                x.trim()
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
                    .to_string()
            })
            .collect();
        let winning: Vec<usize> = nsplit[0].split(' ').map(|x| x.parse().unwrap()).collect();
        let numbers: Vec<usize> = nsplit[1].split(' ').map(|x| x.parse().unwrap()).collect();

        let have_winning: Vec<usize> = winning
            .iter()
            .filter(|x| numbers.iter().any(|y| y == *x))
            .copied()
            .collect();
        let num_winning = have_winning.len();
        let score = if have_winning.is_empty() {
            0
        } else {
            2_usize.pow((have_winning.len() - 1).try_into().unwrap())
        };
        total_score += score;
        cards.push(num_winning);
    }

    let total_score_ = start_scratchcard_game(cards);
    println!("{} {}", total_score, total_score_);
}
