mod common;

use std::time::Duration;
use criterion::{Criterion, criterion_group, criterion_main};
use sorting_algos_test::{SortingAlgorithm, Generator};

fn sort_6000_values(c: &mut Criterion) {
	meassure!(
		name = "Sort of 6,000 values",
		case = "Worst",
		amount_vals = 6_000,
		bencher = c
	);
}

fn sort_60000_values(c: &mut Criterion) {
	meassure!(
		name = "Sort of 60,000 values",
		case = "Worst",
		amount_vals = 60_000,
		bencher = c
	);
}

fn sort_600000_values(c: &mut Criterion) {
	meassure!(
		name = "Sort of 600,000 values",
		case = "Worst",
		amount_vals = 600_000,
		bencher = c
	);
}

fn sort_6000000_values(c: &mut Criterion) {
	meassure!(
		name = "Sort of 6,000,000 values",
		case = "Worst",
		amount_vals = 6_000_000,
		bencher = c
	);
}

fn sort_60000000_values(c: &mut Criterion) {
	meassure!(
		name = "Sort of 60,000,000 values",
		case = "Worst",
		amount_vals = 60_000_000,
		bencher = c
	);
}

criterion_group!(
	name = benches;
	config = Criterion::default()
		.measurement_time(Duration::from_millis(500))
		.sample_size(10);
	targets = sort_6000_values, sort_60000_values, sort_600000_values,
		sort_6000000_values, sort_60000000_values
);

criterion_main!(benches);
