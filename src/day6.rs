use std::{env, fs::read_to_string};

use ahash::{AHashMap, AHashSet};
use anyhow::{bail, Context, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Position {
	Empty,
	Obstacle,
	Guard,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	pub fn next(self) -> Self {
		match self {
			Self::Up => Self::Right,
			Self::Right => Self::Down,
			Self::Down => Self::Left,
			Self::Left => Self::Up,
		}
	}

	pub fn apply(self, loc: (usize, usize)) -> (usize, usize) {
		match self {
			Direction::Up => (loc.0, loc.1 - 1),
			Direction::Down => (loc.0, loc.1 + 1),
			Direction::Left => (loc.0 - 1, loc.1),
			Direction::Right => (loc.0 + 1, loc.1),
		}
	}
}

impl TryFrom<char> for Position {
	type Error = anyhow::Error;

	fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
		Ok(match value {
			'.' => Self::Empty,
			'#' => Self::Obstacle,
			'^' => Self::Guard,
			x => bail!("invalid: {:?}", x),
		})
	}
}

fn walk(
	map: &AHashMap<(usize, usize), Position>,
	mut loc: (usize, usize),
) -> AHashSet<(usize, usize)> {
	let mut dir = Direction::Up;

	let mut set = AHashSet::new();
	set.insert(loc);

	while let Some(thing) = map.get(&dir.apply(loc)) {
		match thing {
			Position::Empty | Position::Guard => {
				loc = dir.apply(loc);
			}
			Position::Obstacle => {
				dir = dir.next();
			}
		}
		set.insert(loc);
	}

	set
}

pub fn part1(map: &AHashMap<(usize, usize), Position>) -> Result<usize> {
	let loc = map
		.iter()
		.find_map(|x| (*x.1 == Position::Guard).then_some(*x.0))
		.context("no guard found")?;

	Ok(walk(map, loc).len())
}

pub fn part2(map: &AHashMap<(usize, usize), Position>) -> Result<usize> {
	let loc = map
		.iter()
		.find_map(|x| (*x.1 == Position::Guard).then_some(*x.0))
		.context("no guard found")?;

	let mut set = AHashMap::new();

	let cnt = walk(map, loc)
		.into_iter()
		.filter(|obstacle_loc| {
			let mut dir = Direction::Up;

			let mut loc = loc;

			set.insert(loc, dir);

			while let Some(thing) = map.get(&dir.apply(loc)) {
				match (thing, dir.apply(loc) == *obstacle_loc) {
					(Position::Empty | Position::Guard, false) => {
						loc = dir.apply(loc);
					}
					(Position::Empty | Position::Guard, true) | (Position::Obstacle, _) => {
						dir = dir.next();
					}
				}
				if let Some(olddir) = set.get(&loc) {
					if dir == *olddir {
						set.clear();
						return true;
					}
				} else {
					set.insert(loc, dir);
				}
			}
			set.clear();
			false
		})
		.count();
	Ok(cnt)
}

pub fn parse(input: &str) -> Result<AHashMap<(usize, usize), Position>> {
	let input = read_to_string(input).context("failed to read input")?;

	let ret: Result<_> = input
		.lines()
		.enumerate()
		.flat_map(|(y, val)| {
			val.char_indices()
				.map(move |(x, val)| Ok(((x + 1, y + 1), Position::try_from(val)?)))
		})
		.collect();

	ret
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let data = parse(&input)?;

	println!("part1 {}", part1(&data)?);
	println!("part2 {}", part2(&data)?);
	Ok(())
}
