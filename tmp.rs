use std::{env, fs::read_to_string};

use anyhow::{Context, Result};
pub fn part1() -> Result<()> {
	Ok(())
}

pub fn part2() -> Result<()> {
	Ok(())
}

pub fn parse(input: &str) -> Result<()> {
	let input = read_to_string(input).context("failed to read input")?;

	Ok(())
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let data = parse(&input)?;

	//println!("part1 {}", part1()?);
	//println!("part2 {}", part2()?);
	Ok(())
}
