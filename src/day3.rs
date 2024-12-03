use std::{env, fs::read_to_string};

use anyhow::{Context, Result};
use regex::Regex;

pub fn part1(str: &str) -> Result<usize> {
	let regex = Regex::new(r"mul\(([0-9]*),([0-9]*)\)")?;

	let ret = regex
		.captures_iter(str)
		.filter_map(|x| {
			Some(
				x.get(1)?.as_str().parse::<usize>().ok()?
					* x.get(2)?.as_str().parse::<usize>().ok()?,
			)
		})
		.sum();

	Ok(ret)
}

pub fn part2(str: &str) -> Result<usize> {
	let regex = Regex::new(r"mul\(([0-9]*),([0-9]*)\)|don?'?t?\(\)")?;

	let mut enabled = true;
	let ret = regex
		.captures_iter(str)
		.filter_map(|x| match x.get(0).map(|x| x.as_str()) {
			Some("do()") => {
				enabled = true;
				None
			}
			Some("don't()") => {
				enabled = false;
				None
			}
			_ if enabled => Some(
				x.get(1)?.as_str().parse::<usize>().ok()?
					* x.get(2)?.as_str().parse::<usize>().ok()?,
			),
			_ => None,
		})
		.sum();

	Ok(ret)
}

pub fn parse(input: &str) -> Result<String> {
	read_to_string(input).context("failed to read input")
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let data = parse(&input)?;

	println!("part1 {}", part1(&data)?);
	println!("part2 {}", part2(&data)?);
	Ok(())
}
