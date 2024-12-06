use std::{env, fs::read_to_string};

use anyhow::{Context, Result};

pub fn part1(input: &[Vec<char>]) -> Result<usize> {
	let mut amt = 0;
	let str = "XMAS";
	for y in 0..input.len() {
		for x in 0..input[0].len() {
			for dx in -1isize..=1isize {
				for dy in -1isize..=1isize {
					let mut matches = true;
					for i in 0..str.len() {
						let mut x = x as isize;
						let mut y = y as isize;
						x += dx * i as isize;
						y += dy * i as isize;
						matches = matches
							&& input
								.get(y as usize)
								.and_then(|vec| vec.get(x as usize).copied())
								.unwrap_or_default() == str.chars().nth(i).unwrap();
					}
					if matches {
						amt += 1;
					}
				}
			}
		}
	}

	Ok(amt)
}

fn maybeget(input: &[Vec<char>], x: usize, y: usize) -> Option<char> {
	input.get(y).and_then(|v| v.get(x).copied())
}

fn wrap(x: usize, y: isize) -> usize {
	x.wrapping_add_signed(y)
}

fn maybematches(input: &[Vec<char>], x: usize, y: usize) -> Option<bool> {
	let topleft = maybeget(input, wrap(x, -1), wrap(y, -1))?;
	let topright = maybeget(input, wrap(x, 1), wrap(y, -1))?;
	let bottomleft = maybeget(input, wrap(x, -1), wrap(y, 1))?;
	let bottomright = maybeget(input, wrap(x, 1), wrap(y, 1))?;
	let mid = input[y][x];

	Some(
		match (
			format!("{topleft}{mid}{bottomright}").as_str(),
			format!("{topright}{mid}{bottomleft}").as_str(),
		) {
			("MAS", "MAS") => true,
			("SAM", "MAS") => true,
			("MAS", "SAM") => true,
			("SAM", "SAM") => true,
			_ => false,
		},
	)
}

pub fn part2(input: &[Vec<char>]) -> Result<usize> {
	let mut amt = 0;
	for y in 0..input.len() {
		for x in 0..input[0].len() {
			if maybematches(input, x, y).is_some_and(|x| x) {
				amt += 1;
			}
		}
	}

	Ok(amt)
}

pub fn parse(input: &str) -> Result<Vec<Vec<char>>> {
	let input = read_to_string(input).context("failed to read input")?;

	Ok(input.lines().map(|x| x.chars().collect()).collect())
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let data = parse(&input)?;

	println!("part1 {}", part1(&data)?);
	println!("part2 {}", part2(&data)?);
	Ok(())
}
