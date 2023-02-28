use rand::{rngs::ThreadRng, Rng};
use rayon::prelude::*;

/// A `sorting algorithm` implementation set.
///
/// The `sorting algorithm` implementation set exposes the implementation
/// of the following sorting algorithms:
///
/// - Cocktailsort
/// - Radixsort
/// - Binary tree sort
/// - QuickSort
///
/// > Those implementations are meant to ***test*** its efficiency sorting a
/// > [`Vec`] with a variatic length,
/// > filled with arbitrary random numbers.
#[derive(Debug)]
pub struct SortingAlgorithm {
	rng_gen: ThreadRng
}

impl SortingAlgorithm {
	/// Creates a new `SortingAlgorithm` instance with a lazily-initialized
	/// thread-local random number generator.
	/// See [`ThreadRng`].
	pub fn new() -> Self {
		Self { rng_gen: rand::thread_rng() }
	}

	/// The *Divide and Conquer* `Quicksort` algorithm implementation.
	/// `Note`: This implementation sorts the vector in-place.
	/// `Note`: This implemention takes as `pivot` the last element in the vector.
	pub fn quicksort<T>(&self, vec: &mut [T])
		where T: PartialOrd
	{
		self._quicksort(vec, 0, (vec.len() - 1) as isize);
	}

	/// The *Sub-routine based* `Radix sort` algorithm implementation.
	/// `Note`: This implementation sorts the vector in-place.
	pub fn radixsort(&self, vec: &mut [isize]) {
		// getting the most significant value
		 let max = match vec.par_iter().max() {
			Some(&x) => x as usize,
			None => return,
		};

		// Make radix a power of 2 close to arr.len() for optimal runtime
		let radix = vec.len().next_power_of_two();

		// Counting so by each digit from least to most significant
		let mut place = 1;
		let mut counter;
		while place <= max {
			let digit_of = |x| x as usize / place % radix;
			// Count digit occurrences
			counter = vec![0; radix];
			for &x in vec.iter() {
				counter[digit_of(x)] += 1;
			}

			// Compute last index of each digit
			for i in 1..radix {
				counter[i] += counter[i - 1];
			}

			// Write elements to their new indices
			for &x in vec.to_owned().iter().rev() {
				counter[digit_of(x)] -= 1;
				vec[counter[digit_of(x)]] = x;
			}

			place *= radix;
		}
	}

	/// Creates a new [`Vec`]
	/// filled with an specified `amount` of random numbers.
	pub fn build_vec(&mut self, amont: isize) -> Vec<isize> {
		(0..amont)
			.map(|_| self.rng_gen.gen())
			.collect()
	}

	/// A [`SortingAlgorithm`]'s quicksort function helper.
	fn _quicksort<T>(&self, vec: &mut [T], low: isize, high: isize)
		where
			T: PartialOrd
	{
		if low <= high {
			let partition_idx = self._partition(vec, 0, high);

			self._quicksort(vec, low, partition_idx - 1);
			self._quicksort(vec, partition_idx + 1, high);
		}
	}

	/// A [`SortingAlgorithm`]'s _partition function helper.
	fn _partition<T>(&self, vec: &mut [T], low: isize, high: isize) -> isize
		where
			T: PartialOrd
	{
		// taking the last value as pivot
		let pivot = high;
		let mut i = low - 1;

		for j in low..=high - 1 {
			if vec[j as usize] <= vec[pivot as usize] {
				i += 1;
				vec.swap(i as usize, j as usize);
			}
		}

		vec.swap((i + 1) as usize, pivot as usize);

		i + 1
	}
}
