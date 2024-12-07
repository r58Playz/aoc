use std::{env, fs::read_to_string, iter::repeat_n};

use anyhow::{Context, Result};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Op {
	Add,
	Mult,
	Concat,
	ConcatAlt,
}

impl Op {
	pub fn apply(self, l: usize, r: usize) -> usize {
		match self {
			Self::Add => l + r,
			Self::Mult => l * r,
			Self::Concat => {
				if r < 10 { return l * 10 + r };
				if r < 100 { return l * 100 + r };
				if r < 1000 { return l * 1000 + r };

				let mut pow = 10;
				let mut rtemp = r;
				while rtemp > 9 {
					rtemp /= 10;
					pow *= 10;
				}
				l * pow + r
			}
			Self::ConcatAlt => l * 10_usize.pow(r.ilog10() + 1) + r,
		}
	}
}

fn run(data: &[usize], acc: usize, target: usize, ops: &[Op]) -> bool {
	if data.len() == 1 {
		for op in ops {
			if op.apply(acc, data[0]) == target {
				return true;
			}
		}
		false
	} else {
		for op in ops {
			if run(&data[1..], op.apply(acc, data[0]), target, ops) {
				return true;
			}
		}
		false
	}
}

// much slower. there's just too many combinations
#[allow(dead_code)]
fn runthreaded(data: &[usize], _acc: usize, target: usize, ops: &[Op]) -> bool {
	repeat_n(ops.iter(), data.len() - 1)
		.multi_cartesian_product()
		.par_bridge()
		.map(|x| {
			let mut acc = data[0];
			for (i, op) in x.iter().enumerate() {
				acc = op.apply(acc, data[i + 1]);
				if acc > target {
					return false;
				}
			}
			acc == target
		})
		.any(|x| x)
}

pub fn part1_single(data: &[(usize, Vec<usize>)]) -> Result<usize> {
	Ok(data
		.iter()
		.filter(|x| run(&x.1, 0, x.0, &[Op::Add, Op::Mult]))
		.map(|x| x.0)
		.sum())
}

pub fn part2_single(data: &[(usize, Vec<usize>)]) -> Result<usize> {
	Ok(data
		.iter()
		.filter(|x| run(&x.1, 0, x.0, &[Op::Add, Op::Mult, Op::Concat]))
		.map(|x| x.0)
		.sum())
}

pub fn part1(data: &[(usize, Vec<usize>)]) -> Result<usize> {
	Ok(data
		.par_iter()
		.filter(|x| run(&x.1, 0, x.0, &[Op::Add, Op::Mult]))
		.map(|x| x.0)
		.sum())
}

pub fn part2(data: &[(usize, Vec<usize>)]) -> Result<usize> {
	Ok(data
		.par_iter()
		.filter(|x| run(&x.1, 0, x.0, &[Op::Add, Op::Mult, Op::Concat]))
		.map(|x| x.0)
		.sum())
}

pub fn parse(input: &str) -> Result<Vec<(usize, Vec<usize>)>> {
	let input = read_to_string(input).context("failed to read input")?;

	input
		.lines()
		.map(|x| {
			let mut loc = x.split(": ");
			let res = loc
				.next()
				.context("failed to get result")?
				.parse()
				.context("failed to parse result")?;
			let nums = loc
				.next()
				.context("failed to get nums")?
				.split(' ')
				.map(|x| x.parse().context("failed to parse num"))
				.try_collect()?;

			Ok((res, nums))
		})
		.try_collect()
}

pub fn main() -> Result<()> {
	let input = env::args().nth(1).context("no input")?;
	let data = parse(&input)?;

	println!("part1 {}", part1(&data)?);
	println!("part2 {}", part2(&data)?);
	Ok(())
}
