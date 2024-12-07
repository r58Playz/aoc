use aoc::{
	day6::{parse, part1, part2},
	get_path,
};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
	let input = parse(get_path!("/day6/input")).unwrap();
	let example = parse(get_path!("/day6/example")).unwrap();

	c.bench_with_input(
		BenchmarkId::new("day6/part1", "example"),
		&example,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part1(x), BatchSize::SmallInput),
	);
	c.bench_with_input(
		BenchmarkId::new("day6/part2", "example"),
		&example,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part2(x), BatchSize::SmallInput),
	);

	c.bench_with_input(
		BenchmarkId::new("day6/part1", "input"),
		&input,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part1(x), BatchSize::SmallInput),
	);
	c.bench_with_input(
		BenchmarkId::new("day6/part2", "input"),
		&input,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part2(x), BatchSize::SmallInput),
	);
}

criterion_group!(day6, criterion_benchmark);
criterion_main!(day6);
