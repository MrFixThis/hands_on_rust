#[macro_export]
macro_rules! meassure {
    (name = $name:expr, case = $case:expr, amount_vals = $a:expr, bencher = $b:ident) => {
		let mut gen = Generator::<u8>::new();
		let coll = match $case {
			"Best" => gen.gen_organized_vec($a, |a, b| a.cmp(b)),
			"Averange" => gen.gen_organized_vec($a, |a, b| b.cmp(a)),
			"Worst" => gen.gen_shuffled_vec($a),
			unk => panic!(r#"unknown case "{}""#, unk)
		};

		let (mut coll2, mut coll3, mut coll4) = (coll.clone(), coll.clone(), coll.clone());
		let mut group = $b.benchmark_group(format!("{} - {} case", $name, $case));
		group.bench_function(
			"Quicksort",
			|b| b.iter(|| SortingAlgorithm::quicksort(
					criterion::black_box(&mut coll4)
				)
			)
		);

		group.bench_function(
			"Radix sort",
			|b| b.iter(|| SortingAlgorithm::radix_sort(
				criterion::black_box(&mut coll3)
				)
			)
		);

		group.bench_function(
			"Cocktail sort",
			|b| b.iter(|| SortingAlgorithm::cocktail_sort(
					criterion::black_box(&mut coll2)
				)
			)
		);

		group.bench_function(
			"Tree sort",
			|b| b.iter(|| SortingAlgorithm::tree_sort(
					criterion::black_box(&coll)
				)
			)
		);

		group.finish();
	};
}
