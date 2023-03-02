#![feature(is_sorted)]

use std::{fmt::Debug, marker::PhantomData, cmp::Ordering};

use binary_search_tree::BinarySearchTree;
use rand::{rngs::ThreadRng, Rng, distributions::Standard, prelude::Distribution};
use rayon::prelude::*;

/// A `sorting algorithm` implementation set.
///
/// The `sorting algorithm` implementation set exposes the implementation
/// of the following sorting algorithms:
///
/// - QuickSort
/// - Radix sort
/// - Cocktail sort
/// - Tree sort
///
/// `Note`: Those implementations are meant to ***test*** its efficiency in
/// its *best*, *worst* and *averange* cases at
/// sorting a [`Vec`]
/// with a variatic length, filled with arbitrary numbers with the next
/// characteristics:
///
/// - Sorted in a ascending manner **(best case)**
/// - Completly shuffled **(averange case)**
/// - Sorted in a descending manner **(worst case)**
///
/// On the other hand, is good to know that this implementations are neither
/// the most optimized nor robust ones.
#[derive(Debug)]
pub struct SortingAlgorithm;

impl SortingAlgorithm {
	/// The *Divide and Conquer* `Quicksort` algorithm implementation.
	/// `Note`: This implementation sorts the collection in-place.
	/// `Note`: This implemention takes as `pivot` the last element in the
	/// collection.
	pub fn quicksort<T>(coll: &mut [T])
		where T: PartialOrd + Debug
	{
		if coll.len() == 0 { panic!("quicksort: the collection is empty") }

		Self::_quicksort(coll, 0, (coll.len() - 1) as isize);
	}

	/// The *Sub-routine based* `Radix sort` algorithm implementation.
	/// `Note`: This implementation does not sort the collection in-place.
	pub fn radix_sort(coll: &mut [u8]) {
		// getting the most significant value
		// to know the number of elements
		 let max = match coll.par_iter().max() {
			Some(&x) => x as usize,
			None =>  panic!("radix sort: the collection is empty")
		};

		// Make radix a power of 2 close to arr.len() for optimal runtime
		let radix = coll.len().next_power_of_two();

		let mut place = 1;
		let mut counter;
		// Counting so by each digit from least to most significant
		while place <= max {
			// Count digit occurrences
			counter = vec![0; radix];
			for &x in coll.iter() {
				counter[(x as usize / place) % radix] += 1;
			}

			// Computing the final index of each digit
			for i in 1..radix {
				// adding the previos element with the current one
				counter[i] += counter[i - 1];
			}

			// Write elements to their new indices
			for &x in coll.to_owned().iter().rev() { // this can be better, much better...
				counter[(x as usize / place) % radix] -= 1;
				coll[counter[(x as usize / place) % radix]] = x;
			}

			place *= radix;
		}
	}

	/// The *Linear* `Cocktail sort` algorithm implementation.
	/// `Note`: This implementation sorts the collection in-place.
	pub fn cocktail_sort<T>(coll: &mut [T])
		where
			T: PartialOrd + Debug
	{
		if coll.len() == 0 { panic!("cocktail sort: the collection is empty") }

		let mut swapped = true;
		let mut start: usize = 0;
		let mut end = coll.len();

		while swapped == true {
			// comparing from bottom to top
			swapped = Self::_one_way_bubble_sort(coll, start..end-1);
			// if nothing was swapped, so the collection is sorted
			if !swapped { break }

			// decressing end by one because the last item now is in its correct
			// spot
			end -= 1;

			// comparing from top to bottom
			swapped = Self::_one_way_bubble_sort(coll, (start..end-1).rev());

			// increassing start by one because the first item now is in its
			// correct spot
			start += 1;
		}
	}

	/// The *Divide and Conquer* `Tree sort` algorithm implementation.
	/// `Note`: This implementation does not sort the collection in-place.
	pub fn tree_sort<T>(coll: &[T]) -> Vec<&T>
		where
			T: PartialOrd + Ord + Debug,
	{
		if coll.len() == 0 { panic!("tree sort: the collection is empty") }

		// creating a BST with the references of coll's elements
		let bst = BinarySearchTree::from_iter(coll.into_iter());

		// creating a vector from bst with the references of coll's items
		// sorted in in-order manner
		bst.into_sorted_vec()
	}

	/// A [`SortingAlgorithm`]'s quicksort function helper.
	fn _quicksort<T>(vec: &mut [T], low: isize, high: isize)
		where
			T: PartialOrd
	{
		if low < high {
			// getting the partition index
			let partition_idx = Self::_partition(vec, 0, high);

			// left half
			Self::_quicksort(vec, low, partition_idx - 1);
			// right half
			Self::_quicksort(vec, partition_idx + 1, high);
		}
	}

	/// A partition-based algorithm helper function.
	/// This implemention returns a partition index when a `pivot` finally
	/// was took on its place in the collection.
	fn _partition<T>(coll: &mut [T], low: isize, high: isize) -> isize
		where
			T: PartialOrd
	{
		let pivot_idx = high;
		//index of the smallest element
		let mut i = low - 1;

		for j in low..=high - 1 {
			if coll[j as usize] <= coll[pivot_idx as usize] {
				i += 1;
				coll.swap(i as usize, j as usize);
			}
		}

		// taking the pivot in its correct place
		coll.swap((i + 1) as usize, pivot_idx as usize);

		i + 1
	}

	/// A bubble sort-based algorithm helper function.
	/// This implementation is just a one-way bubble sort that returns
	/// a `true` if something was changed in the collection, or `false` otherwise.
	fn _one_way_bubble_sort<T, R>(coll: &mut [T], range: R) -> bool
	where
		T: PartialOrd,
		R: Iterator<Item = usize>
	{
		let mut swapped = false;
		for i in range {
			if coll[i] > coll[i + 1] {
				coll.swap(i, i + 1);
				if !swapped { swapped = true };
			}
		}

		swapped
	}
}

/// A `test vector generator`.
///
/// This generator crates three kinds of container vectors:
///
/// - A vector with elements sorted in an ascending manner.
/// - A vector with elements completly shuffled.
/// - A vector with elements sorted in a descending manner.
///
/// `Note`: The resulting vector is meant to test
/// the sorting efficiency of the above algorithms.
#[derive(Debug)]
pub struct Generator<T> {
	rng_gen: ThreadRng,
	_marker: PhantomData<T>
}

impl<T> Generator<T>
	where
		Standard: Distribution<T>,
		T: PartialOrd + Ord
{
	/// Creates a new `Generator` instance with a lazily-initialized
	/// thread-local random number generator.
	/// See [`ThreadRng`] for details.
	pub fn new() -> Self {
		Self {
			rng_gen: rand::thread_rng(),
			_marker: PhantomData,
		}
	}

	/// Creates a new [`Vec`]
	/// filled with an specified `amount` of numbers organized either in an
	/// `ascending` or `descending` manner.
	pub fn gen_organized_vec<F>(&mut self, amount: usize, f: F) -> Vec<T>
		where
			F: FnMut(&T, &T) -> Ordering
	{
		let mut vec: Vec<T> = self.gen_shuffled_vec(amount);
		vec.sort_by(f);
		vec
	}

	/// Creates a new [`Vec`]
	/// filled with an specified `amount` of numbers completly shuffled.
	pub fn gen_shuffled_vec(&mut self, amount: usize) -> Vec<T> {
		(1..=amount)
			.map(|_| self.rng_gen.gen())
			.collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn quicksort_sorts_random_collection() {
		let mut gen = Generator::new();
		let mut vec: Vec<i8> = gen.gen_shuffled_vec(6745);

		SortingAlgorithm::quicksort(&mut vec);
		assert!(vec.is_sorted());
	}

	#[test]
	fn radix_sort_sorts_random_collection() {
		let mut gen = Generator::new();
		let mut vec: Vec<u8> = gen.gen_shuffled_vec(4345);

		SortingAlgorithm::radix_sort(&mut vec);
		assert!(vec.is_sorted());
	}

	#[test]
	fn cocktail_sort_sorts_random_collection() {
		let mut gen = Generator::new();
		let mut vec: Vec<i8> = gen.gen_shuffled_vec(2131);

		SortingAlgorithm::cocktail_sort(&mut vec);
		assert!(vec.is_sorted());
	}

	#[test]
	fn tree_sort_sorts_random_collection() {
		let mut gen = Generator::new();
		let vec: Vec<i8> = gen.gen_shuffled_vec(3764);

		let sorted_vec = SortingAlgorithm::tree_sort(&vec);
		assert!(sorted_vec.is_sorted());
	}
}
