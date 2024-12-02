use aoc::day1::{parse, part1, part2};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
	let input = parse("/home/r58playz/Documents/aoc/day1/input").unwrap();
	let example = parse("/home/r58playz/Documents/aoc/day1/example").unwrap();

	c.bench_with_input(
		BenchmarkId::new("day1/part1", "example"),
		&example,
		|b, input| {
			b.iter_batched_ref(
				|| input.clone(),
				|(ref mut l, ref mut r)| part1(l, r),
				BatchSize::SmallInput,
			)
		},
	);
	c.bench_with_input(
		BenchmarkId::new("day1/part2", "example"),
		&example,
		|b, input| {
			b.iter_batched_ref(
				|| input.clone(),
				|(ref mut l, ref mut r)| part2(l, r),
				BatchSize::SmallInput,
			)
		},
	);

	c.bench_with_input(
		BenchmarkId::new("day1/part1", "input"),
		&input,
		|b, input| {
			b.iter_batched_ref(
				|| input.clone(),
				|(ref mut l, ref mut r)| part1(l, r),
				BatchSize::SmallInput,
			)
		},
	);
	c.bench_with_input(
		BenchmarkId::new("day1/part2", "input"),
		&input,
		|b, input| {
			b.iter_batched_ref(
				|| input.clone(),
				|(ref mut l, ref mut r)| part2(l, r),
				BatchSize::SmallInput,
			)
		},
	);
}

criterion_group!(day1, criterion_benchmark);
criterion_main!(day1);
