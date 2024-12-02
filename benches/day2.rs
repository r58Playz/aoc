use aoc::day2::{parse, part1, part2};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
	let input = parse("/home/r58playz/Documents/aoc/day2/input").unwrap();
	let example = parse("/home/r58playz/Documents/aoc/day2/example").unwrap();

	c.bench_with_input(
		BenchmarkId::new("day2/part1", "example"),
		&example,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part1(x), BatchSize::SmallInput),
	);
	c.bench_with_input(
		BenchmarkId::new("day2/part2", "example"),
		&example,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part2(x), BatchSize::SmallInput),
	);

	c.bench_with_input(
		BenchmarkId::new("day2/part1", "input"),
		&input,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part1(x), BatchSize::SmallInput),
	);
	c.bench_with_input(
		BenchmarkId::new("day2/part2", "input"),
		&input,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part2(x), BatchSize::SmallInput),
	);
}

criterion_group!(day2, criterion_benchmark);
criterion_main!(day2);
