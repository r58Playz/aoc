use std::{env, fs::read_to_string};

use anyhow::{Context, Result};
use rustc_hash::FxHashSet;

pub fn wrappingoffset(
	p: (isize, isize),
	v: (isize, isize),
	min: (isize, isize),
	max: (isize, isize),
) -> (isize, isize) {
	let mut new = (p.0 + v.0, p.1 + v.1);
	if new.0 < min.0 {
		new.0 += max.0;
	}
	if new.1 < min.1 {
		new.1 += max.1;
	}
	if new.0 >= max.0 {
		new.0 -= max.0;
	}
	if new.1 >= max.1 {
		new.1 -= max.1;
	}
	new
}

pub fn part1(
	mut robots: Vec<((isize, isize), (isize, isize))>,
) -> Result<usize> {
	let size = (101, 103);

	for _ in 0..100 {
		for robot in robots.iter_mut() {
			robot.0 = wrappingoffset(robot.0, robot.1, (0, 0), size);
		}
	}

	let midx = size.0 / 2;
	let midy = size.1 / 2;

	#[rustfmt::skip]
	let mut score = (
		/*0*/0, /*1*/0,
		/*2*/0, /*3*/0
	);

	for (pos, _) in robots {
		if pos.0 < midx && pos.1 < midy {
			score.0 += 1;
		} else if pos.0 > midx && pos.1 < midy {
			score.1 += 1;
		} else if pos.0 < midx && pos.1 > midy {
			score.2 += 1;
		} else if pos.0 > midx && pos.1 > midy {
			score.3 += 1;
		}
	}

	Ok(score.0 * score.1 * score.2 * score.3)
}

pub fn part2(
	mut robots: Vec<((isize, isize), (isize, isize))>,
) -> Result<usize> {
	let size = (101, 103);
	let mut i = 0;
	let mut seen = FxHashSet::default();

	loop {
		i += 1;
		for robot in &mut robots {
			robot.0 = wrappingoffset(robot.0, robot.1, (0, 0), size);
			seen.insert(robot.0);
		}

		if seen.len() == robots.len() {
			return Ok(i);
		}
		seen.clear();
	}
}

pub fn parse(input: &str) -> Result<Vec<((isize, isize), (isize, isize))>> {
	let input = read_to_string(input).context("failed to read input")?;

	input
		.lines()
		.map(|x| {
			let (p, v) = x.split_once(" ").context("failed to split p/v")?;
			let p = p.split_once("=").context("failed to split p by equals")?.1;
			let v = v.split_once("=").context("failed to split v by equals")?.1;

			let (px, py) = p.split_once(",").context("failed to split p by comma")?;
			let (vx, vy) = v.split_once(",").context("failed to split v by comma")?;

			Ok(((px.parse()?, py.parse()?), (vx.parse()?, vy.parse()?)))
		})
		.collect()
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let data = parse(&input)?;

	println!("part1 {}", part1(data.clone())?);
	println!("part2 {}", part2(data.clone())?);
	Ok(())
}
