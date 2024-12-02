use std::{env, fs::read_to_string};

use anyhow::{Context, Result};

pub fn parse(input: &str) -> Result<Vec<Vec<i32>>> {
	let input = read_to_string(input).context("failed to read input")?;

	let out = input
		.lines()
		.map(|x| {
			x.split_ascii_whitespace()
				.map(|x| x.parse().map_err(Into::into))
				.collect::<Result<Vec<i32>>>()
		})
		.collect::<Result<Vec<_>>>()?;

	Ok(out)
}

pub fn validate(x: &[i32]) -> bool {
	(x.is_sorted() || x.iter().rev().is_sorted())
		&& x.iter()
			.map_windows(|[a, b]| {
				let x = a.abs_diff(**b);
				(1..=3).contains(&x)
			})
			.all(|x| x)
}

pub fn part1(data: &[Vec<i32>]) -> usize {
	data.iter().filter(|x| validate(x)).count()
}

pub fn part2(data: &[Vec<i32>]) -> usize {
	// magic number - max items in input is 8, subtract 1 and square
	let mut tmp = Vec::with_capacity(49);
	data.iter()
		.map(|x| (validate(x), x))
		.filter(|(ok, x)| {
			if *ok {
				true
			} else {
				for i in 0..x.len() {
					x.iter()
						.enumerate()
						.filter_map(|(idx, x)| (idx != i).then_some(x))
						.collect_into(&mut tmp);

					if validate(&tmp) {
						return true;
					}
				}
				false
			}
		})
		.count()
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let data = parse(&input)?;

	let part1 = part1(&data);
	println!("part1 {part1}");

	let part2 = part2(&data);
	println!("part2 {part2}");

	Ok(())
}
