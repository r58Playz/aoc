use std::{env, fs::read_to_string};

use anyhow::{Context, Result};
use rustc_hash::FxHashMap;

fn digits(mut x: u128) -> usize {
	let mut size = 1;
	while x > 9 {
		x /= 10;
		size += 1;
	}
	size
}

fn split(x: u128, size: usize) -> (u128, u128) {
	let size = 10u128.pow((size / 2) as u32);
	(x / size, x % size)
}

fn evolve_recurse(
	n: u128,
	depth: usize,
	target: usize,
	cache: &mut FxHashMap<(usize, u128), u128>,
) -> u128 {
	if let Some(x) = cache.get(&(depth, n)) {
		return *x;
	}

	let ret;
	let newdepth = depth + 1;

	if depth == target {
		return 1;
	}
	if n == 0 {
		ret = evolve_recurse(1, newdepth, target, cache);
	} else if let digits = digits(n)
		&& digits % 2 == 0
	{
		let (l, r) = split(n, digits);
		ret =
			evolve_recurse(l, newdepth, target, cache) + evolve_recurse(r, newdepth, target, cache);
	} else {
		ret = evolve_recurse(n * 2024, newdepth, target, cache);
	}
	cache.insert((depth, n), ret);
	ret
}

pub fn part1(data: &[u128]) -> Result<usize> {
	let mut cache = FxHashMap::default();
	Ok(data
		.iter()
		.map(|x| evolve_recurse(*x, 0, 25, &mut cache) as usize)
		.sum())
}

pub fn part2(data: &[u128]) -> Result<usize> {
	let mut cache = FxHashMap::default();
	Ok(data
		.iter()
		.map(|x| evolve_recurse(*x, 0, 75, &mut cache) as usize)
		.sum())
}

pub fn parse(input: &str) -> Result<Vec<u128>> {
	let input = read_to_string(input).context("failed to read input")?;

	input
		.trim()
		.split(' ')
		.map(|x| x.parse().context("failed to parse num"))
		.collect()
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let data = parse(&input)?;

	println!("part1 {}", part1(&data)?);
	println!("part2 {}", part2(&data)?);
	Ok(())
}
