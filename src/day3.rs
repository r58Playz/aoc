#![allow(dead_code)]
use std::io::Read;

fn find_start_of_current_num(row: Vec<char>, start_idx: usize) -> usize {
    let mut idx = start_idx;

    loop {
        if idx == 0 {
            break;
        }
        if is_end_of_num(row[idx - 1]) {
            break;
        }
        idx -= 1;
    }

    idx
}

fn is_valid_symbol(ch: char) -> bool {
    "!\"#$%&'()*+,-/:;<=>?@[\\]^_`{|}~".find(ch).is_some()
}

fn is_end_of_num(ch: char) -> bool {
    is_valid_symbol(ch) || ch == '.'
}

fn parse_num(vec: Vec<char>, start: usize, end: usize) -> i32 {
    vec[start..end]
        .iter()
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

fn parse_num_tuple(vec: Vec<char>, num: &(usize, usize)) -> i32 {
    parse_num(vec, num.0, num.1+1)
}

fn clamp(x: usize, l: usize, h: usize) -> usize {
    if x < l {
        l
    } else if x > h {
        h
    } else {
        x
    }
}

pub fn run() {
    let mut file = std::fs::File::open("day3/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut inputmatrix: Vec<Vec<char>> =
        contents.split('\n').map(|x| x.chars().collect()).collect();
    inputmatrix.pop();

    let mut valid_numbers: Vec<Vec<(usize, usize)>> =
        (0..inputmatrix.len()).map(|_| Vec::new()).collect();

    let mut total = 0;

    for (ri, row) in inputmatrix.iter().enumerate() {
        let mut is_valid_num = false;
        for (ci, ch) in row.iter().enumerate() {
            if ch.is_ascii_digit() {
                if (ri != 0 && is_valid_symbol(inputmatrix[ri - 1][ci]))
                    || (ri != inputmatrix.len() - 1 && is_valid_symbol(inputmatrix[ri + 1][ci]))
                    || (ci != 0 && is_valid_symbol(inputmatrix[ri][ci - 1]))
                    || (ci != row.len() - 1 && is_valid_symbol(inputmatrix[ri][ci + 1]))
                    || (ri != 0 && ci != 0 && is_valid_symbol(inputmatrix[ri - 1][ci - 1]))
                    || (ri != inputmatrix.len() - 1
                        && ci != 0
                        && is_valid_symbol(inputmatrix[ri + 1][ci - 1]))
                    || (ri != 0
                        && ci != row.len() - 1
                        && is_valid_symbol(inputmatrix[ri - 1][ci + 1]))
                    || (ri != inputmatrix.len() - 1
                        && ci != row.len() - 1
                        && is_valid_symbol(inputmatrix[ri + 1][ci + 1]))
                {
                    is_valid_num = true;
                }
                if (ci == row.len() - 1 || is_end_of_num(inputmatrix[ri][ci + 1])) && is_valid_num {
                    let current_num = find_start_of_current_num(row.to_vec(), ci);
                    total += parse_num(row.to_vec(), current_num, ci + 1);
                    valid_numbers[ri].push((current_num, ci));
                }
            } else {
                is_valid_num = false;
            }
        }
    }

    let mut total_gear_ratio = 0;

    for (ri, row) in inputmatrix.iter().enumerate() {
        for (ci, ch) in row.iter().enumerate() {
            if *ch == '*' {
                let mut intersecting_numbers: Vec<(usize, &(usize, usize))> = Vec::new();
                for (pri, prow) in valid_numbers.iter().enumerate().take(ri + 2).skip(ri - 1)
                {
                    for pci in clamp(ci - 1, 0, row.len() - 1)..clamp(ci + 1, 0, row.len() - 1)+1 {
                        let iter = prow 
                            .iter()
                            .filter(|x| pci >= x.0 && pci <= x.1)
                            .map(|x| (pri, x));
                        intersecting_numbers.extend(iter);
                    }
                }
                
                intersecting_numbers.sort();
                intersecting_numbers.dedup_by_key(|x| (x.0, x.1.0, x.1.1));

                if intersecting_numbers.len() == 2 {
                    println!("{:?}", intersecting_numbers);
                    total_gear_ratio += parse_num_tuple(
                        inputmatrix[intersecting_numbers[0].0].clone(),
                        intersecting_numbers[0].1,
                    ) * parse_num_tuple(
                        inputmatrix[intersecting_numbers[1].0].clone(),
                        intersecting_numbers[1].1,
                    );
                }
            }
        }
    }

    println!("{} {}", total, total_gear_ratio);
}
