#![feature(
	iter_map_windows,
	iter_collect_into,
	iterator_try_collect,
	array_windows,
	unsigned_signed_diff,
	let_chains
)]

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, ValueEnum};

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Day {
	Day1,
	Day2,
	Day3,
	Day4,
	Day5,
	Day6,
	Day7,
	Day8,
	Day9,
	Day10,
	Day11,
	Day12,
	Day13,
	Day14,
}

#[derive(Clone, Debug, Parser)]
/// Advent Of Code 2024 CLI.
struct Cli {
	/// Day to run.
	day: Day,
	/// Path to input from Advent Of Code site.
	input: PathBuf,
}

fn main() -> Result<()> {
	let args = Cli::parse();
	let input = args.input.to_str().unwrap();

	match args.day {
		Day::Day1 => day1::main(input),
		Day::Day2 => day2::main(input),
		Day::Day3 => day3::main(input),
		Day::Day4 => day4::main(input),
		Day::Day5 => day5::main(input),
		Day::Day6 => day6::main(input),
		Day::Day7 => day7::main(input),
		Day::Day8 => day8::main(input),
		Day::Day9 => day9::main(input),
		Day::Day10 => day10::main(input),
		Day::Day11 => day11::main(input),
		Day::Day12 => day12::main(input),
		Day::Day13 => day13::main(input),
		Day::Day14 => day14::main(input),
	}
}
