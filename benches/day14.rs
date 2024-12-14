use aoc::{
	day14::{parse, part1, part2},
	get_path,
};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
	let input = parse(get_path!("/day14/input")).unwrap();
	let example = parse(get_path!("/day14/example")).unwrap();

	c.bench_with_input(
		BenchmarkId::new("day14/part1", "example"),
		&example,
		|b, input| b.iter_batched(|| input.clone(), part1, BatchSize::SmallInput),
	);
	c.bench_with_input(
		BenchmarkId::new("day14/part2", "example"),
		&example,
		|b, input| b.iter_batched(|| input.clone(), part2, BatchSize::SmallInput),
	);

	c.bench_with_input(
		BenchmarkId::new("day14/part1", "input"),
		&input,
		|b, input| b.iter_batched(|| input.clone(), part1, BatchSize::SmallInput),
	);
	c.bench_with_input(
		BenchmarkId::new("day14/part2", "input"),
		&input,
		|b, input| b.iter_batched(|| input.clone(), part2, BatchSize::SmallInput),
	);
}

criterion_group!(day14, criterion_benchmark);
criterion_main!(day14);
