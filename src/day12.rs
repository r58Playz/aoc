use std::{env, fs::read_to_string};

use anyhow::{Context, Result};
use rustc_hash::FxHashMap;

pub fn build(
	data: &FxHashMap<(usize, usize), char>,
	loc: (usize, usize),
	curr: &mut (Vec<(usize, usize)>, usize),
	target: char,
) -> bool {
	if let Some(char) = data.get(&loc)
		&& *char == target
	{
		curr.0.push(loc);
		let next = (loc.0 - 1, loc.1);
		if !curr.0.contains(&next) && !build(data, next, curr, target) {
			curr.1 += 1;
		}
		let next = (loc.0 + 1, loc.1);
		if !curr.0.contains(&next) && !build(data, next, curr, target) {
			curr.1 += 1;
		}
		let next = (loc.0, loc.1 - 1);
		if !curr.0.contains(&next) && !build(data, next, curr, target) {
			curr.1 += 1;
		}
		let next = (loc.0, loc.1 + 1);
		if !curr.0.contains(&next) && !build(data, next, curr, target) {
			curr.1 += 1;
		}
		true
	} else {
		false
	}
}

pub fn part1(data: &FxHashMap<(usize, usize), char>) -> Result<usize> {
	let mut regions: FxHashMap<char, Vec<(Vec<(usize, usize)>, usize)>> = FxHashMap::default();

	for (loc, char) in data {
		let regionlist = regions.entry(*char).or_default();
		if regionlist.iter().any(|x| x.0.iter().any(|x| x == loc)) {
			continue;
		}

		let mut region = (Vec::new(), 0);
		build(data, *loc, &mut region, *char);
		regionlist.push(region);
	}

	Ok(regions
		.iter()
		.map(|x| {
			let ret = x.1.iter().map(|x| x.0.len() * x.1).sum::<usize>();
			ret
		})
		.sum())
}

fn offset(x: (usize, usize), o: (isize, isize)) -> (usize, usize) {
	(x.0.wrapping_add_signed(o.0), x.1.wrapping_add_signed(o.1))
}

fn neighbors(x: (usize, usize)) -> [(usize, usize); 4] {
	[
		offset(x, (1, 0)),
		offset(x, (-1, 0)),
		offset(x, (0, -1)),
		offset(x, (0, 1)),
	]
}

pub fn build2(
	data: &FxHashMap<(usize, usize), char>,
	loc: (usize, usize),
	curr: &mut (Vec<(usize, usize)>, Vec<(usize, usize, usize)>, isize),
	target: char,
) {
	if let Some(char) = data.get(&loc)
		&& *char == target
	{
		curr.0.push(loc);
		let next = (loc.0 - 1, loc.1);
		if !curr.0.contains(&next) {
			build2(data, next, curr, target);
		}
		let next = (loc.0 + 1, loc.1);
		if !curr.0.contains(&next) {
			build2(data, next, curr, target);
		}
		let next = (loc.0, loc.1 - 1);
		if !curr.0.contains(&next) {
			build2(data, next, curr, target);
		}
		let next = (loc.0, loc.1 + 1);
		if !curr.0.contains(&next) {
			build2(data, next, curr, target);
		}

		for (i, neighbor) in neighbors(loc).iter().enumerate() {
			if data.get(neighbor).is_none_or(|x| *x != target) {
				curr.2 += 1;

				curr.1.push((i, neighbor.0, neighbor.1));

				for other in neighbors(*neighbor) {
					if curr.1.contains(&(i, other.0, other.1)) {
						// if there's already a neighbor in that direction we are doublecounting
						curr.2 -= 1;
					}
				}
			}
		}
	}
}

pub fn part2(data: &FxHashMap<(usize, usize), char>) -> Result<isize> {
	let mut regions: FxHashMap<
		char,
		Vec<(Vec<(usize, usize)>, Vec<(usize, usize, usize)>, isize)>,
	> = FxHashMap::default();

	for (loc, char) in data {
		let regionlist = regions.entry(*char).or_default();
		if regionlist.iter().any(|x| x.0.iter().any(|x| x == loc)) {
			continue;
		}

		let mut region = Default::default();
		build2(data, *loc, &mut region, *char);
		regionlist.push(region);
	}

	Ok(regions
		.iter()
		.map(|x| {
			let ret = x.1.iter().map(|x| x.0.len() as isize * x.2).sum::<isize>();
			ret
		})
		.sum())
}

pub fn parse(input: &str) -> Result<FxHashMap<(usize, usize), char>> {
	let input = read_to_string(input).context("failed to read input")?;

	Ok(input
		.lines()
		.enumerate()
		.flat_map(|(y, x)| x.char_indices().map(move |(x, val)| ((x + 1, y + 1), val)))
		.collect())
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let data = parse(&input)?;

	println!("part1 {}", part1(&data)?);
	println!("part2 {}", part2(&data)?);
	Ok(())
}
