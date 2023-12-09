#![allow(dead_code)]
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Read;

#[derive(Clone, Debug)]
struct Hand {
    hand_type: HandType,
    hand: String,
}

impl Hand {
    pub fn from_string(str: String) -> Hand {
        use HandType::*;
        let hand_type: HandType;

        let str_vec: Vec<_> = str.chars().collect();
        let mut str_map = HashMap::new();

        for char in str_vec {
            *str_map.entry(char).or_insert(0) += 1;
        }

        if let Some(num_jokers_ref) = str_map.get(&'J') {
            let num_jokers = *num_jokers_ref;
            str_map.remove(&'J');

            if let Some(highest_key) = str_map.iter().max_by_key(|entry| entry.1) {
                str_map
                    .entry(*highest_key.0)
                    .and_modify(|v| *v += num_jokers);
            } else {
                // line 951: JJJJJ 131
                *str_map.entry('A').or_insert(0) += 5;
            }
        }

        let mut str_map_freqs: Vec<_> = str_map.iter().map(|x| x.1).copied().collect();
        str_map_freqs.sort();

        if str_map.len() == 5 {
            hand_type = HighCard;
        } else if str_map.len() == 1 {
            hand_type = FiveKind;
        } else if str_map_freqs == vec![1, 4] {
            hand_type = FourKind;
        } else if str_map_freqs == vec![2, 3] {
            hand_type = FullHouse;
        } else if str_map_freqs == vec![1, 1, 3] {
            hand_type = ThreeKind;
        } else if str_map_freqs == vec![1, 2, 2] {
            hand_type = TwoPair;
        } else if str_map_freqs == vec![1, 1, 1, 2] {
            hand_type = OnePair;
        } else {
            hand_type = None;
        }

        Hand {
            hand_type,
            hand: str,
        }
    }

    pub fn get_char_points(x: char) -> usize {
        match x {
            'J' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("impossible"),
        }
    }

    pub fn get_hand_points(x: &Hand) -> usize {
        x.hand_type as usize
    }

    pub fn compare_hand(&self, other: &Hand) -> Ordering {
        let hpoints = Hand::get_hand_points(self).cmp(&Hand::get_hand_points(other));

        if hpoints != Ordering::Equal {
            return hpoints;
        } else {
            for (self_c, other_c) in self.hand.chars().zip(other.hand.chars()) {
                let cpoints = Hand::get_char_points(self_c).cmp(&Hand::get_char_points(other_c));
                if cpoints != Ordering::Equal {
                    return cpoints;
                }
            }
        }

        Ordering::Equal
    }
}

#[derive(Copy, Clone, Debug)]
enum HandType {
    None = 0,
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

pub fn run() {
    let mut file = std::fs::File::open("day7/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut data: Vec<_> = contents.split('\n').collect();

    data.pop();

    let mut hands: Vec<_> = data
        .iter()
        .map(|x| {
            let mut y = x.split(' ');
            (
                Hand::from_string(y.next().unwrap().to_string()),
                y.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    hands.sort_by(|a, b| a.0.compare_hand(&b.0));

    println!(
        "{:?}",
        hands
            .iter()
            .enumerate()
            .map(|(i, x)| x.1 * (i + 1))
            .sum::<usize>()
    );
}
