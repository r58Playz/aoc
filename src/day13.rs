use std::{env, fs::read_to_string};

use anyhow::{Context, Result};

type Pt = (usize, usize);

pub fn part1(data: &[(Pt, Pt, Pt)]) -> Result<usize> {
	let mut cnt = 0;

	for (aoffset, boffset, goal) in data {
		let a1 = aoffset.0 as f64;
		let a2 = aoffset.1 as f64;
		let b1 = boffset.0 as f64;
		let b2 = boffset.1 as f64;
		let g1 = goal.0 as f64;
		let g2 = goal.1 as f64;

		let x = (g1 * b2 - b1 * g2) / (a1 * b2 - a2 * b1);
		let y = (g1 - a1 * x) / b1;

		if x.floor() == x && y.floor() == y {
			cnt += x as usize * 3 + y as usize;
		}
	}

	Ok(cnt)
}

pub fn part2(data: &[(Pt, Pt, Pt)]) -> Result<usize> {
	let mut cnt = 0;

	for (aoffset, boffset, goal) in data {
		let goal = (goal.0 + 10000000000000, goal.1 + 10000000000000);

		let a1 = aoffset.0 as f64;
		let a2 = aoffset.1 as f64;
		let b1 = boffset.0 as f64;
		let b2 = boffset.1 as f64;
		let g1 = goal.0 as f64;
		let g2 = goal.1 as f64;

		let x = (g1 * b2 - b1 * g2) / (a1 * b2 - a2 * b1);
		let y = (g1 - a1 * x) / b1;

		if x.floor() == x && y.floor() == y {
			cnt += x as usize * 3 + y as usize;
		}
	}

	Ok(cnt)
}

fn parsept(str: &str, delim: &str) -> Result<(usize, usize)> {
	let str = str.split_once(": ").context("failed to split line")?.1;
	let (l, r) = str
		.split_once(", ")
		.context("failed to split point by comma")?;
	let l = l
		.split_once(delim)
		.context("failed to split left by delim")?
		.1;
	let r = r
		.split_once(delim)
		.context("failed to split right by delim")?
		.1;
	Ok((
		l.parse().context("failed to parse left")?,
		r.parse().context("failed to parse right")?,
	))
}

pub fn parse(input: &str) -> Result<Vec<(Pt, Pt, Pt)>> {
	let input = read_to_string(input).context("failed to read input")?;

	input
		.split("\n\n")
		.map(|x| {
			let mut lines = x.lines();
			let a = parsept(lines.next().context("failed to get btn a")?, "+")
				.context("failed to parse btn a")?;
			let b = parsept(lines.next().context("failed to get btn a")?, "+")
				.context("failed to parse btn a")?;
			let prize = parsept(lines.next().context("failed to get btn a")?, "=")
				.context("failed to parse btn a")?;

			anyhow::Ok((a, b, prize))
		})
		.collect()
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let data = parse(&input)?;

	dbg!(part1(&data)?);
	dbg!(part2(&data)?);
	Ok(())
}
