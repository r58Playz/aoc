use std::{env, fs::read_to_string};

use anyhow::{bail, Context, Result};
use itertools::Itertools;
use rustc_hash::FxHashMap;

fn dist(l: &(usize, usize), r: &(usize, usize)) -> (isize, isize) {
	(
		r.0.checked_signed_diff(l.0).unwrap(),
		r.1.checked_signed_diff(l.1).unwrap(),
	)
}
fn inbounds(x: &(usize, usize), size: &(usize, usize)) -> bool {
	x.0 < size.0 && x.1 < size.1
}

pub fn part1(
	(data, size): &(FxHashMap<char, Vec<(usize, usize)>>, (usize, usize)),
) -> Result<usize> {
	let mut map: FxHashMap<(usize, usize), char> = FxHashMap::default();

	for (thing, locs) in data {
		for (loc, otherloc) in locs.iter().tuple_combinations() {
			let dist = dist(loc, otherloc);
			let node1 = (
				loc.0.wrapping_add_signed(-dist.0),
				loc.1.wrapping_add_signed(-dist.1),
			);
			let node2 = (
				otherloc.0.wrapping_add_signed(dist.0),
				otherloc.1.wrapping_add_signed(dist.1),
			);
			if inbounds(&node1, size) {
				map.insert(node1, *thing);
			}
			if inbounds(&node2, size) {
				map.insert(node2, *thing);
			}
		}
	}

	Ok(map.len())
}

pub fn part2(
	(data, size): &(FxHashMap<char, Vec<(usize, usize)>>, (usize, usize)),
) -> Result<usize> {
	let mut map: FxHashMap<(usize, usize), char> = FxHashMap::default();

	for (thing, locs) in data {
		for (loc, otherloc) in locs.iter().tuple_combinations() {
			let dist = dist(loc, otherloc);

			let mut node1 = *loc;
			let mut node2 = *otherloc;
			while inbounds(&node1, size) {
				map.insert(node1, *thing);
				node1 = (
					node1.0.overflowing_add_signed(-dist.0).0,
					node1.1.overflowing_add_signed(-dist.1).0,
				);
			}
			while inbounds(&node2, size) {
				map.insert(node2, *thing);
				node2 = (
					node2.0.overflowing_add_signed(dist.0).0,
					node2.1.overflowing_add_signed(dist.1).0,
				);
			}
		}
	}

	Ok(map.len())
}

pub fn parse(input: &str) -> Result<(FxHashMap<char, Vec<(usize, usize)>>, (usize, usize))> {
	let input = read_to_string(input).context("failed to read input")?;

	let mut map = FxHashMap::default();
	input
		.lines()
		.enumerate()
		.flat_map(|(y, x)| x.char_indices().map(move |(x, val)| ((x, y), val)))
		.try_for_each(|(loc, val)| {
			match val {
				'.' => {}
				v if v.is_ascii_alphanumeric() => {
					map.entry(v)
						.and_modify(|x: &mut Vec<(usize, usize)>| x.push(loc))
						.or_insert_with(|| vec![loc]);
				}
				v => bail!("invalid char {:?}", v),
			}
			Ok(())
		})?;

	Ok((
		map,
		(
			input.lines().next().context("input empty")?.len(),
			input.lines().count(),
		),
	))
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let data = parse(&input)?;

	println!("part1 {}", part1(&data)?);
	println!("part2 {}", part2(&data)?);
	Ok(())
}
