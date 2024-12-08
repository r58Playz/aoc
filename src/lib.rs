#![feature(
	iter_map_windows,
	iter_collect_into,
	iterator_try_collect,
	array_windows,
	unsigned_signed_diff
)]

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

#[macro_export]
macro_rules! get_path {
	($path:literal) => {
		concat!(env!("CARGO_MANIFEST_DIR"), $path)
	};
}
