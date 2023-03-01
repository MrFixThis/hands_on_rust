#![feature(test)]

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use sorting_algos_test::SortingAlgorithm;

fn sort_6000_values(c: &mut Criterion) {
	let mut set = SortingAlgorithm::new();
	let mut vec: Vec<u8> = set.build_vec(6_000);
	let mut group = c.benchmark_group("Sort of 6,000 values");

    group.bench_function(
		"Quicksort",
		|b| b.iter(|| set.quicksort(criterion::black_box(&mut vec)))
	);

    group.bench_function(
		"Radix sort",
		|b| b.iter(|| set.radix_sort(criterion::black_box(&mut vec)))
	);

    group.bench_function(
		"Cocktail sort",
		|b| b.iter(|| set.cocktail_sort(criterion::black_box(&mut vec)))
	);

    group.bench_function(
		"Tree sort",
		|b| b.iter(|| set.tree_sort(criterion::black_box(&vec)))
	);

	group.finish();
}

fn sort_60000_values(c: &mut Criterion) {
	let mut set = SortingAlgorithm::new();
	let mut vec: Vec<u8> = set.build_vec(60_000);
	let mut group = c.benchmark_group("Sort of 60,000 values");

    group.bench_function(
		"Quicksort",
		|b| b.iter(|| set.quicksort(criterion::black_box(&mut vec)))
	);

    group.bench_function(
		"Radix sort",
		|b| b.iter(|| set.radix_sort(criterion::black_box(&mut vec)))
	);

    group.bench_function(
		"Cocktail sort",
		|b| b.iter(|| set.cocktail_sort(criterion::black_box(&mut vec)))
	);

    group.bench_function(
		"Tree sort",
		|b| b.iter(|| set.tree_sort(criterion::black_box(&vec)))
	);

	group.finish();
}

fn sort_600000_values(c: &mut Criterion) {
	let mut set = SortingAlgorithm::new();
	let mut vec: Vec<u8> = set.build_vec(600_000);
	let mut group = c.benchmark_group("Sort of 600,000 values");

    group.bench_function(
		"Quicksort",
		|b| b.iter(|| set.quicksort(criterion::black_box(&mut vec)))
	);

    group.bench_function(
		"Radix sort",
		|b| b.iter(|| set.radix_sort(criterion::black_box(&mut vec)))
	);

    group.bench_function(
		"Cocktail sort",
		|b| b.iter(|| set.cocktail_sort(criterion::black_box(&mut vec)))
	);

    group.bench_function(
		"Tree sort",
		|b| b.iter(|| set.tree_sort(criterion::black_box(&vec)))
	);

	group.finish();
}

fn sort_6000000_values(c: &mut Criterion) {
	let mut set = SortingAlgorithm::new();
	let mut vec: Vec<u8> = set.build_vec(6_000_000);
	let mut group = c.benchmark_group("Sort of 6,000,000 values");

    group.bench_function(
		"Quicksort",
		|b| b.iter(|| set.quicksort(criterion::black_box(&mut vec)))
	);

    group.bench_function(
		"Radix sort",
		|b| b.iter(|| set.radix_sort(criterion::black_box(&mut vec)))
	);

    group.bench_function(
		"Cocktail sort",
		|b| b.iter(|| set.cocktail_sort(criterion::black_box(&mut vec)))
	);

    group.bench_function(
		"Tree sort",
		|b| b.iter(|| set.tree_sort(criterion::black_box(&vec)))
	);

	group.finish();
}

fn sort_60000000_values(c: &mut Criterion) {
	let mut set = SortingAlgorithm::new();
	let mut vec: Vec<u8> = set.build_vec(60_000_000);
	let mut group = c.benchmark_group("Sort of 60,000,000 values");

    group.bench_function(
		"Quicksort",
		|b| b.iter(|| set.quicksort(criterion::black_box(&mut vec)))
	);

    group.bench_function(
		"Radix sort",
		|b| b.iter(|| set.radix_sort(criterion::black_box(&mut vec)))
	);

    group.bench_function(
		"Cocktail sort",
		|b| b.iter(|| set.cocktail_sort(criterion::black_box(&mut vec)))
	);

    group.bench_function(
		"Tree sort",
		|b| b.iter(|| set.tree_sort(criterion::black_box(&vec)))
	);

	group.finish();
}

criterion_group!(
	name = benches;
	config = Criterion::default()
		.measurement_time(Duration::from_millis(1000))
		.sample_size(10);
	targets = sort_6000_values, sort_60000_values, sort_600000_values,
		sort_6000000_values, sort_60000000_values
);

criterion_main!(benches);
