//! This crate contains the solution of the excercise with the following
//! statement:
//!
//! ```text
//! Using the [brute force method](https://en.wikipedia.org/wiki/Proof_by_exhaustion)
//! find the mode, median, mean and the maximum value inside a collection
//! of numeric elements.
//! ```
//!
//! Note: The implementations that follows the brute force method tend
//! to be inefficient, so the implementations written for this solution
//! are not the best at all and does not take advantage of the Rust's true power.
//! With that said, all the code present here is written with learning purposes
//! of the brute force algoritimic tecnique and the exhaustion concept.

mod walker;

use walker::Report;

fn main() {
    let my_coll: Vec<usize> = vec![5, 6, 7, 8, 15, 20, 67, 71, 5, 6];
    let rep = Report::from(my_coll);
    println!("{rep}");
}
