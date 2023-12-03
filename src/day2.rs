#![allow(dead_code)]
use std::io::Read;

struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
    possible: bool,
}

struct Game {
    id: i32,
    min_cubeset: CubeSet,
    possible: bool
}

impl CubeSet {
    pub fn empty() -> CubeSet {
        CubeSet {
            red: 0,
            green: 0,
            blue: 0,
            possible: true,
        }
    }

    pub fn parse_from_line(line: String) -> CubeSet {
        let mut red_cnt = -1;
        let mut green_cnt = -1;
        let mut blue_cnt = -1;
        for cube in line.split(',') {
            let split_cube: Vec<&str> = cube.trim().split(' ').collect();
            match split_cube[1] {
                "red" => red_cnt = split_cube[0].parse().unwrap(),
                "green" => green_cnt = split_cube[0].parse().unwrap(),
                "blue" => blue_cnt = split_cube[0].parse().unwrap(),
                _ => unreachable!(),
            }
        }
        CubeSet {
            red: red_cnt,
            green: green_cnt,
            blue: blue_cnt,
            possible: red_cnt <= 12 && green_cnt <= 13 && blue_cnt <= 14,
        }
    }
}

impl Game {
    pub fn parse_from_line(line: String) -> Game {
        let line_split: Vec<String> = line.split(':').map(String::from).collect();
        let id: i32 = line_split[0]
            .trim()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let subgames: Vec<CubeSet> = line_split[1]
            .split("; ")
            .map(|subgame| CubeSet::parse_from_line(subgame.trim().to_string()))
            .collect();

        let possible = subgames.iter().all(|game| game.possible);

        let min_cubeset = subgames.iter().fold(CubeSet::empty(), |acc, x| {
            CubeSet {
                red: if acc.red > x.red { acc.red } else { x.red },
                green: if acc.green > x.green { acc.green } else { x.green },
                blue: if acc.blue > x.blue { acc.blue } else { x.blue },
                possible: true
            }
        });

        Game { id, possible, min_cubeset }
    }
}

pub fn run() {
    let mut file = std::fs::File::open("day2/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut possible_game_ids_total = 0;
    let mut powers_of_minimum_cubesets_total = 0;

    for line in contents.split('\n') {
        if line.is_empty() {continue}
        let game = Game::parse_from_line(line.to_string());
        if game.possible {
            possible_game_ids_total += game.id;
        }
        powers_of_minimum_cubesets_total += game.min_cubeset.red * game.min_cubeset.green * game.min_cubeset.blue;
    }

    println!("{}", possible_game_ids_total);
    println!("{}", powers_of_minimum_cubesets_total);
}
