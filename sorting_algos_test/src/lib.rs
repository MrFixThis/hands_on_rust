#![feature(is_sorted)]

use std::fmt::Debug;

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
/// `Note`: Those implementations are meant to ***test*** its efficiency at
/// sorting a [`Vec`]
/// with a variatic length, filled with arbitrary random numbers.
#[derive(Debug)]
pub struct SortingAlgorithm {
	rng_gen: ThreadRng
}

impl SortingAlgorithm {
	/// Creates a new `SortingAlgorithm` instance with a lazily-initialized
	/// thread-local random number generator.
	/// See [`ThreadRng`] for details.
	pub fn new() -> Self {
		Self { rng_gen: rand::thread_rng() }
	}

	/// The *Divide and Conquer* `Quicksort` algorithm implementation.
	/// `Note`: This implementation sorts the collection in-place.
	/// `Note`: This implemention takes as `pivot` the last element in the
	/// collection.
	pub fn quicksort<T>(&self, coll: &mut [T])
		where T: PartialOrd + Debug
	{
		if coll.len() == 0 { panic!("quicksort: the collection is empty") }

		self._quicksort(coll, 0, (coll.len() - 1) as isize);
	}

	/// The *Sub-routine based* `Radix sort` algorithm implementation.
	/// `Note`: This implementation does not sort the collection in-place.
	pub fn radix_sort(&self, coll: &mut [u8]) {
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
	pub fn cocktail_sort<T>(&self, coll: &mut [T])
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
	pub fn tree_sort<'a, T>(&self, coll: &'a [T]) -> Vec<&'a T>
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

	/// Creates a new [`Vec`]
	/// filled with an specified `amount` of random numbers.
	/// `Note`: The resulting vector is meant to test
	/// the sorting efficiency of the above algorithm.
	pub fn build_vec<T>(&mut self, amont: usize) -> Vec<T>
		where
			Standard: Distribution<T>
	{
		(0..amont)
			.map(|_| self.rng_gen.gen())
			.collect()
	}

	/// A [`SortingAlgorithm`]'s quicksort function helper.
	fn _quicksort<T>(&self, vec: &mut [T], low: isize, high: isize)
		where
			T: PartialOrd
	{
		if low < high {
			// getting the partition index
			let partition_idx = Self::_partition(vec, 0, high);

			// left half
			self._quicksort(vec, low, partition_idx - 1);
			// right half
			self._quicksort(vec, partition_idx + 1, high);
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn quicksort_sorts_random_collection() {
		let mut set = SortingAlgorithm::new();
		let mut vec: Vec<i8> = set.build_vec(6745);

		set.quicksort(&mut vec);
		assert!(vec.is_sorted());
	}

	#[test]
	fn radix_sort_sorts_random_collection() {
		let mut set = SortingAlgorithm::new();
		let mut vec: Vec<u8> = set.build_vec(4345);

		set.radix_sort(&mut vec);
		assert!(vec.is_sorted());
	}

	#[test]
	fn cocktail_sort_sorts_random_collection() {
		let mut set = SortingAlgorithm::new();
		let mut vec: Vec<i8> = set.build_vec(2131);

		set.cocktail_sort(&mut vec);
		assert!(vec.is_sorted());
	}

	#[test]
	fn tree_sort_sorts_random_collection() {
		let mut set = SortingAlgorithm::new();
		let vec: Vec<i8> = set.build_vec(3764);

		let sorted_vec = set.tree_sort(&vec);
		assert!(sorted_vec.is_sorted());
	}
}
