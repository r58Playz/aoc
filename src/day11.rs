use std::{env, fs::read_to_string, time::Instant};

use anyhow::{Context, Result};
use rustc_hash::FxHashMap;

fn digits(mut x: u64) -> u8 {
	let mut size = 1;
	while x > 9 {
		x /= 10;
		size += 1;
	}
	size
}

fn split(x: u64, size: u8) -> (u64, u64) {
	let size = 10u64.pow(u32::from(size / 2));
	(x / size, x % size)
}

fn evolve_recurse<const MAX: usize>(
	n: u64,
	depth: usize,
	cache: &mut FxHashMap<u64, [u64; MAX]>,
) -> u64 {
	if depth == MAX {
		return 1;
	}

	if let Some(x) = cache.get(&n)
		&& x[depth] != 0
	{
		return x[depth];
	}

	let ret;
	let newdepth = depth + 1;

	let digits = digits(n);
	if n == 0 {
		ret = evolve_recurse(1, newdepth, cache);
	} else if digits % 2 == 0 {
		let (l, r) = split(n, digits);
		ret = evolve_recurse(l, newdepth, cache) + evolve_recurse(r, newdepth, cache);
	} else {
		ret = evolve_recurse(n * 2024, newdepth, cache);
	}

	cache.entry(n).or_insert([0; MAX])[depth] = ret;

	ret
}

pub fn part1(data: &[u64]) -> Result<u64> {
	let mut cache = FxHashMap::default();
	Ok(data
		.iter()
		.map(|x| evolve_recurse::<25>(*x, 0, &mut cache))
		.sum())
}

pub fn part2(data: &[u64]) -> Result<u64> {
	let mut cache = FxHashMap::default();
	Ok(data
		.iter()
		.map(|x| evolve_recurse::<75>(*x, 0, &mut cache))
		.sum())
}

pub fn parse(input: &str) -> Result<Vec<u64>> {
	let input = read_to_string(input).context("failed to read input")?;

	input
		.trim()
		.split(' ')
		.map(|x| x.parse().context("failed to parse num"))
		.collect()
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let before = Instant::now();
	let data = parse(&input)?;
	dbg!(before.elapsed());

	let before = Instant::now();
	println!("part1 {}", part1(&data)?);
	dbg!(before.elapsed());
	let before = Instant::now();
	println!("part2 {}", part2(&data)?);
	dbg!(before.elapsed());
	Ok(())
}
