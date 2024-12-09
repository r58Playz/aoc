use std::{env, fs::read_to_string, time::Instant};

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

pub fn part1(data: &[Option<u64>]) -> Result<u64> {
	let mut out = data.to_vec();

	while let Some((empty, _)) = out.iter().enumerate().find(|(_, x)| x.is_none()) {
		out.swap_remove(empty);
	}

	Ok(out
		.iter()
		.enumerate()
		.filter_map(|x| x.1.map(|v| x.0 as u64 * v))
		.sum())
}

pub fn part2(data: &[Option<u64>]) -> Result<u64> {
	let mut out = data.to_vec();

	let lastid = data.iter().k_largest(1).next().context("empty")?.unwrap();

	for id in (0..=lastid).rev() {
		let (loc, _) = data
			.iter()
			.enumerate()
			.find(|x| x.1.is_some_and(|x| x == id))
			.unwrap();
		let size = data.iter().filter(|x| x.is_some_and(|x| x == id)).count();

		let mut maybeemptystart = None;
		for (emptystart, _) in out
			.iter()
			.enumerate()
			.filter(|x| x.1.is_none() && x.0 < loc)
		{
			let mut emptysize = 0;
			while let Some(None) = out.get(emptystart + emptysize) {
				emptysize += 1;
			}
			if size <= emptysize {
				maybeemptystart.insert(emptystart);
				break;
			}
		}

		if let Some(emptystart) = maybeemptystart {
			for i in 0..size {
				out.swap(emptystart + i, loc + i);
			}
		}
	}

	Ok(out
		.iter()
		.enumerate()
		.filter_map(|x| x.1.map(|v| x.0 as u64 * v))
		.sum())
}

pub fn parse1(input: &str) -> Result<Vec<Option<u64>>> {
	let input = read_to_string(input).context("failed to read input")?;

	let input: Vec<_> = input
		.chars()
		.filter(|x| x.is_numeric())
		.map(|x| {
			#[allow(clippy::cast_possible_truncation)]
			x.to_digit(10)
				.map(|x| x as u8)
				.ok_or_else(|| anyhow!("failed to parse {:?}", x))
		})
		.try_collect()?;

	let mut vec = Vec::with_capacity(input.iter().map(|x| *x as usize).sum());
	let mut id: u64 = 0;
	let mut free = false;

	for len in input {
		for _ in 0..len {
			vec.push(if free { None } else { Some(id) });
		}
		if !free {
			id += 1;
		}
		free = !free;
	}

	Ok(vec)
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let data1 = parse1(&input)?;

	let start = Instant::now();
	let x = part1(&data1)?;
	let end = Instant::now();
	println!("part1 {} {:?}", x, end - start);
	println!("part2 {}", part2(&data1)?);
	Ok(())
}
