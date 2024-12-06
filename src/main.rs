#![feature(iter_map_windows, iter_collect_into)]

use anyhow::Result;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

fn main() -> Result<()> {
	// day1::main()?;
	// day2::main()?;
	// day3::main()?;
	// day4::main()?;
	// day5::main()?;
	day6::main()?;

	Ok(())
}
