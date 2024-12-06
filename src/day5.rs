use std::{env, fs::read_to_string};

use anyhow::{bail, Context, Result};

fn get_locs_from_update(update: &[usize], rule: &(usize, usize)) -> Option<(usize, usize)> {
	let left = update
		.iter()
		.enumerate()
		.find(|x| *x.1 == rule.0)
		.map(|x| x.0);
	let right = update
		.iter()
		.enumerate()
		.find(|x| *x.1 == rule.1)
		.map(|x| x.0);
	Some((left?, right?))
}

fn validate(rules: &[(usize, usize)], update: &[usize]) -> Result<()> {
	for rule in rules {
		if let Some((leftloc, rightloc)) = get_locs_from_update(update, rule) {
			if leftloc > rightloc {
				bail!("rule {:?} update {:?} not maintained", rule, update);
			}
		}
	}

	Ok(())
}

fn fix(rules: &[(usize, usize)], update: &mut [usize]) {
	while validate(rules, &*update).is_err() {
		for rule in rules {
			if let Some((leftloc, rightloc)) = get_locs_from_update(update, rule) {
				if leftloc > rightloc {
					update.swap(leftloc, rightloc);
				}
			}
		}
	}
}

pub fn part1(rules: Vec<(usize, usize)>, updates: Vec<Vec<usize>>) -> Result<usize> {
	let ret = updates
		.iter()
		.filter_map(|x| validate(&rules, x).ok().map(|_| x))
		.map(|x| x.get(x.len() / 2).unwrap())
		.sum();
	Ok(ret)
}

pub fn part2(rules: Vec<(usize, usize)>, mut updates: Vec<Vec<usize>>) -> Result<usize> {
	let ret = updates
		.iter_mut()
		.filter_map(|x| validate(&rules, x).err().map(|_| x))
		.map(|x| {
			fix(&rules, x);
			x
		})
		.map(|x| x.get(x.len() / 2).unwrap())
		.sum();
	Ok(ret)
}

pub fn parse(input: &str) -> Result<(Vec<(usize, usize)>, Vec<Vec<usize>>)> {
	let input = read_to_string(input).context("failed to read input")?;

	let mut rules = Vec::new();
	let mut updates = Vec::new();

	let mut parsingrules = true;

	for line in input.lines() {
		if line.is_empty() {
			parsingrules = false;
		} else if parsingrules {
			let loc = line.find('|').context("no | in rule")?;
			let (l, r) = line.split_at(loc);
			rules.push((
				l.parse().context("failed to parse rule left")?,
				r.strip_prefix("|")
					.unwrap()
					.parse()
					.context("failed to parse rule right")?,
			));
		} else {
			updates.push(
				line.split(',')
					.map(|x| x.parse().context("failed to parse report num"))
					.collect::<Result<Vec<_>>>()?,
			);
		}
	}

	Ok((rules, updates))
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let (rules, updates) = parse(&input)?;

	println!("part1 {}", part1(rules.clone(), updates.clone())?);
	println!("part2 {}", part2(rules.clone(), updates.clone())?);
	Ok(())
}
