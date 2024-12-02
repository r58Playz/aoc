use std::{env, fs};

use anyhow::{Context, Result};
use nohash_hasher::IntMap;

pub fn parse(input: &str) -> Result<(Vec<i64>, Vec<i64>)> {
	let input = fs::read_to_string(input).context("failed to read input")?;

	let input_lines = input.lines().count();

	let mut left: Vec<i64> = Vec::with_capacity(input_lines);
	let mut right: Vec<i64> = Vec::with_capacity(input_lines);

	for line in input.lines() {
		let mut split = line.split("   ");
		left.push(
			split
				.next()
				.context("failed to get left input")?
				.parse()
				.context("failed to parse input")?,
		);
		right.push(
			split
				.next()
				.context("failed to get left input")?
				.parse()
				.context("failed to parse input")?,
		);
	}

	Ok((left, right))
}

pub fn part1(left: &mut [i64], right: &mut [i64]) -> i64 {
	left.sort_unstable();
	right.sort_unstable();

	let mut dist = 0i64;
	for (litem, ritem) in left.iter().zip(right.iter()) {
		dist += (*litem - *ritem).abs();
	}
	dist
}

pub fn part2(left: &mut [i64], right: &mut [i64]) -> i64 {
	let mut count: IntMap<i64, i64> = IntMap::default();

	for itm in right {
		count.entry(*itm).and_modify(|x| *x += 1).or_insert(1);
	}

	let mut similarity = 0i64;
	for itm in left {
		similarity += *itm * count.get(itm).unwrap_or(&0);
	}
	similarity
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let (mut left, mut right) = parse(&input)?;

	let dist = part1(&mut left, &mut right);
	println!("dist {:?}", dist);

	let similarity = part2(&mut left, &mut right);
	println!("similarity {:?}", similarity);

	Ok(())
}
