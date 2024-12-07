use aoc::{
	day7::{parse, part1, part1_single, part2, part2_single, Op},
	get_path,
};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
	let input = parse(get_path!("/day7/input")).unwrap();
	let example = parse(get_path!("/day7/example")).unwrap();

	c.bench_with_input(
		BenchmarkId::new("day7/part1/multi", "example"),
		&example,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part1(x), BatchSize::SmallInput),
	);
	c.bench_with_input(
		BenchmarkId::new("day7/part2/multi", "example"),
		&example,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part2(x), BatchSize::SmallInput),
	);

	c.bench_with_input(
		BenchmarkId::new("day7/part1/multi", "input"),
		&input,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part1(x), BatchSize::SmallInput),
	);
	c.bench_with_input(
		BenchmarkId::new("day7/part2/multi", "input"),
		&input,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part2(x), BatchSize::SmallInput),
	);

	c.bench_with_input(
		BenchmarkId::new("day7/part1/single", "example"),
		&example,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part1_single(x), BatchSize::SmallInput),
	);
	c.bench_with_input(
		BenchmarkId::new("day7/part2/single", "example"),
		&example,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part2_single(x), BatchSize::SmallInput),
	);

	c.bench_with_input(
		BenchmarkId::new("day7/part1/single", "input"),
		&input,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part1_single(x), BatchSize::SmallInput),
	);
	c.bench_with_input(
		BenchmarkId::new("day7/part2/single", "input"),
		&input,
		|b, input| b.iter_batched_ref(|| input.clone(), |x| part2_single(x), BatchSize::SmallInput),
	);

	let mut concatgroup = c.benchmark_group("concat");
	concatgroup.bench_with_input(
		BenchmarkId::new("day7", "unrolled"),
		&(123, 456),
		|b, input| {
			b.iter_batched_ref(
				|| input,
				|(l, r)| Op::Concat.apply(*l, *r),
				BatchSize::SmallInput,
			);
		},
	);
	concatgroup.bench_with_input(
		BenchmarkId::new("day7", "pow"),
		&(123, 456),
		|b, input| {
			b.iter_batched_ref(
				|| input,
				|(l, r)| Op::ConcatAlt.apply(*l, *r),
				BatchSize::SmallInput,
			);
		},
	);
    concatgroup.finish();
}

criterion_group!(day7, criterion_benchmark);
criterion_main!(day7);
