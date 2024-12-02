#![feature(iter_map_windows, iter_collect_into)]

use anyhow::Result;

pub mod day1;
pub mod day2;

fn main() -> Result<()> {
	// day1::main()?;
	day2::main()?;

	Ok(())
}
