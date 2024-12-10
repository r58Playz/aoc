use std::{env, fs::read_to_string};

use anyhow::{Context, Result};
use rustc_hash::{FxHashMap, FxHashSet};

pub fn calc(
	data: &FxHashMap<(usize, usize), u8>,
	loc: (usize, usize),
	num: u8,
	set: &mut FxHashSet<(usize, usize)>,
) {
	if let Some(x) = data.get(&loc) {
		if *x == 9 && *x == num {
			set.insert(loc);
		} else if *x == num {
			calc(data, (loc.0 - 1, loc.1), num + 1, set);
			calc(data, (loc.0 + 1, loc.1), num + 1, set);
			calc(data, (loc.0, loc.1 - 1), num + 1, set);
			calc(data, (loc.0, loc.1 + 1), num + 1, set);
		}
	}
}

pub fn part1(data: &FxHashMap<(usize, usize), u8>) -> Result<usize> {
	let mut set = FxHashSet::default();
	Ok(data
		.iter()
		.filter(|(x, y)| **y == 0)
		.map(|(x, _)| {
			calc(data, *x, 0, &mut set);
			let len = set.len();
			set.clear();
			len
		})
		.sum())
}

pub fn calc2(data: &FxHashMap<(usize, usize), u8>, loc: (usize, usize), num: u8) -> usize {
	if let Some(x) = data.get(&loc) {
		if *x == 9 && *x == num {
			return 1;
		} else if *x == num {
			return calc2(data, (loc.0 - 1, loc.1), num + 1)
				+ calc2(data, (loc.0 + 1, loc.1), num + 1)
				+ calc2(data, (loc.0, loc.1 - 1), num + 1)
				+ calc2(data, (loc.0, loc.1 + 1), num + 1);
		}
	}
	0
}

pub fn part2(data: &FxHashMap<(usize, usize), u8>) -> Result<usize> {
	Ok(data
		.iter()
		.filter(|(x, y)| **y == 0)
		.map(|(x, _)| calc2(data, *x, 0))
		.sum())
}

pub fn parse(input: &str) -> Result<FxHashMap<(usize, usize), u8>> {
	let input = read_to_string(input).context("failed to read input")?;

	input
		.lines()
		.enumerate()
		.flat_map(|(y, x)| x.char_indices().map(move |(x, val)| ((x + 1, y + 1), val)))
		.map(|x| {
			Ok((
				x.0,
				x.1.to_digit(10).context("failed to parse digit")? as u8,
			))
		})
		.collect()
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let data = parse(&input)?;

	println!("part1 {}", part1(&data)?);
	println!("part2 {}", part2(&data)?);
	Ok(())
}
