use derive_builder::Builder;
use rulinalg::{matrix::{Matrix, BaseMatrix}, vector::Vector};

use crate::error::Result;

pub const DEFAULT_NAMESPACE: [char; 27] = [
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
	'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ' '
];

#[derive(Debug, Default, Builder)]
pub struct Processor<'a> {
	key: &'a str,
	source: &'a str,
	dimension: Option<&'a usize>,
	fill_letter: Option<&'a char>,
	name_space: Option<&'a str>,
}

impl Processor<'_> {
	pub fn cypher(&self) -> Result<Vec<String>> {
		// specification of which namespace to use: either the user supplied
		// namespace or the default one
		let namespace: Vec<_> = match self.name_space {
			Some(ns) => ns.chars().map(|c| c.to_ascii_uppercase()).collect(),
			None => DEFAULT_NAMESPACE.into_iter().collect()
		};

		// cheking if the supplied fill character is inside the namespace
		if let Some(fill) = self.fill_letter {
			is_in_namespace(&fill, &namespace)?;
		}

		// specification of the key's representation matrix's dimension
		let dimension = match self.dimension {
			Some(&dim) => {
				//cheking if the suplied dimension for the key's matrix is square
				if !is_dim_square(dim) {
					return Err(
						format!(
							"the suplied dimension '{dim}' is not square"
						).into()
					);
				}
				dim
			},
			None => ((DEFAULT_NAMESPACE.len() - 1) as f64).sqrt() as usize
		};

		// cheking if the key's length is square. If it is not, the
		// key is filled
		let key = if !is_dim_square(self.key.len()) {
			fill_key(self.key, self.fill_letter.unwrap())?
		} else {
			self.key.to_uppercase()
		};

		// cheking if the source text's length is divisible by the dimension.
		// If it is not, the text is filled
		let source = if self.source.len() % dimension != 0 {
			fill_src(self.source, self.fill_letter.unwrap(), &dimension)?
		} else {
			self.source.to_uppercase()
		};

		// getting the key's matrix representation and its determinant
		let key_mtrx = key_mtrx_repr(&key, &dimension, &namespace);
		let key_det = key_mtrx.clone().det(); // clone because det consumes the
											  // the receiver

		// checking if the supplied key is valid to use for the cypher process
		if key_det == 0.0 || Self::has_any_factor(
			key_det as usize, namespace.len() as usize
		) {
			return Err(
				format!(
					"the specified key cannot be used. [det 0 or has factors with {}]",
					namespace.len()
				 ).into()
			)
		}

		// spliting the source text into as many parts as the dimension of
		// the key's matrix representation, and turns its values
		// into its respective numeric representation inside the namespace
		let src_mtrx = src_text_vec_repr(
			source,
			self.fill_letter.unwrap(),
			&dimension,
			&namespace
		)?;

		// turning the cyphered_parts into its textual representation
		Ok(cypher_src_vecs(
				src_mtrx,
				dimension,
				&key_mtrx,
				&namespace
			)
		)
	}

	/// Checks if a `target number` has at least one factor against any number
	/// specified.
	fn has_any_factor(target: usize, number: usize) -> bool {
		for factor in target..number {
			if target % factor == 0 {
				return true
			}
		}
		false
	}
}

// pub(crate) fn get_inverse_matrix<T>(source: &Matrix<T>) -> Matrix<T>
// where
// 	T: Ord + PartialOrd
// {
//
// }

/// Turns a given (Vec)[alloc::vec] filled with the positions
/// of each character of a `source text`, into its textual representations.
fn cypher_src_vecs(
	src_vec: Vec<f64>,
	dim: usize,
	key_mtrx: &Matrix<f64>,
	namespace: &[char]
) -> Vec<String> {
	// cyphering the source text's matrix

	src_vec
		.chunks(dim)
		.map(|c| (key_mtrx * Vector::new(c) % *&dim as f64))
		.map(|v| {
			v.into_iter()
				.map(|f| namespace.get(f as usize).unwrap())
				.collect::<String>()
		})
		.collect()
}

/// Splits the `source text` into as many parts as the given `dimension`
/// and turns its values into its respective representation inside the
/// namespace specified, and stores it inside a (Vec)[alloc::vec].
fn src_text_vec_repr(
	src: String,
	fill: &char,
	dim: &usize,
	namespace: &[char]
) -> Result<Vec<f64>>
{
	let src_len = src.len();
	let txt = if src_len % dim != 0 {
		fill_src(&src, fill, dim)?
	} else {
		src.to_owned()
	};

	Ok(txt
		.chars()
		.map(|c| char_pos(&c, namespace) as f64) // this can potentially cause a number overflow
		.collect()
	)
}

/// Turns the given `key`'s characters into its positional representation
/// inisde the namespace and stores it inside a
/// (Matrix)[rulinalg::matrix::Matrix].
fn key_mtrx_repr(key: &str, dim: &usize, namespace: &[char]) -> Matrix<f64> {
	// getting the key into its numeric representation
	let vec: Vec<_> = key
		.chars()
		.map(|c| char_pos(&c, namespace) as f64) // this can potentially cause a number overflow
		.collect();

	Matrix::new(*dim, *dim, vec).transpose()
}

/// Fills a given `key` that has not a square length with a specified
/// character that is present in the namespace.
fn fill_key(txt: &str, char: &char) -> Result<String> {
	let txt_len = txt.len();
	let dim = if !is_dim_square(txt_len) {
		turn_perfect_sqrt(txt_len)
	} else {
		txt_len
	};

	Ok(fill_text(txt, char, dim))
}

/// Fills a given `source text` that has not a divisble length by
/// the namespace's `dimension` with a specified character that is present
/// in the namespace.
fn fill_src(txt: &str, char: &char, ns_dim: &usize) -> Result<String> {
	let txt_len = txt.len();
	let dim = if !is_divisble(txt_len, *ns_dim) {
		turn_divisible(txt_len, *ns_dim)
	} else {
		txt_len
	};

	Ok(fill_text(txt, char, dim))
}

/// Fills a given `text` with a specified character `dimension` - `text`.len() times.
fn fill_text(txt: &str, char: &char, dim: usize) -> String {
	let txt_len = txt.len();
	let reps = dim - txt_len as usize;

	if reps != 0 {
		let append = char.to_string().repeat(reps);
		format!("{}{}", txt, append).to_uppercase()
	} else {
		txt.to_owned()
	}
}

/// Retrives the given character's `position` inside the namespace specified.
fn char_pos(char: &char, namespace: &[char]) -> usize {
	namespace.iter().position(|&c| c == char.to_ascii_uppercase()).unwrap()
}

/// Checks if the supplied `dimension` is square.
fn is_dim_square(dim: usize) -> bool {
	let sqrt = (dim as f64).sqrt();

	dim == 0 || dim == 1 || (sqrt * sqrt == dim as f64)
}

/// Checks if the supplied target `dimension` is divisible by another
/// dimension.
fn is_divisble(target: usize, dim: usize) -> bool {
	target % dim == 0
}

/// Turns a given `dimension` a perfect square, upwards the given value.
fn turn_perfect_sqrt(dim: usize) -> usize {
	let mut base = dim as f64;
	let mut sqrt;
	loop {
		sqrt = base.sqrt();
		if sqrt * sqrt == base {
			return base as usize;
		}
		base += 1.0;
	}
}

/// Turns a given target `dimension` divisible by another dimenision.
fn turn_divisible(target: usize, dim: usize) -> usize {
	let mut base = target;
	loop {
		if base % dim == 0 {
			return base;
		}
		base += 1;
	}
}

/// Checks if the supplied `character` is inside the given namespace; if it
/// is not, returns (ProcessingError)[Error::ProcessingError]
fn is_in_namespace(char: &char, namespace: &[char]) -> Result<()> {
	if namespace.into_iter().find(|&c| *c == *char) == None {
		return Err(
			format!(
				"the character '{char}' is not present in the namespace"
			).into()
		);
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn key_with_not_square_length_is_filled() {
		let text = "ABCDE".to_owned();
		let fill_char = 'L';

		assert_eq!(
			fill_key(&text, &fill_char).unwrap(),
			"ABCDELLLL"
		);
	}

	#[test]
	fn text_with_not_divisible_length_is_filled() {
		let ns_dim = ((DEFAULT_NAMESPACE.len() - 1) as f64).sqrt() as usize;
		let text = "ABCD".to_owned();
		let fill_char = 'E';

		assert_eq!(fill_src(&text, &fill_char, &ns_dim).unwrap(), "ABCDE");
	}

	#[test]
	fn key_is_turned_into_matrix_representation() {
		let key = "ABCDEFGHI";
		let dim = (key.len() as f64).sqrt() as usize;

		assert_eq!(
			key_mtrx_repr(&key, &dim, &DEFAULT_NAMESPACE),
			Matrix::new(dim, dim, vec![0.0, 3.0, 6.0, 1.0, 4.0, 7.0, 2.0, 5.0, 8.0])
		)
	}

	#[test]
	fn source_text_is_turned_into_vec_repr() {
		let txt = "ABCDEFGHI".to_owned();
		let dim = (txt.len() as f64).sqrt() as usize;
		let fill_char = 'L';

		assert_eq!(
			src_text_vec_repr(txt, &fill_char, &dim, &DEFAULT_NAMESPACE).unwrap(),
			vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
		);
	}

	#[test]
	fn square_length_source_text_parts_are_turned_into_cyphered_parts() {
		let namespace = &DEFAULT_NAMESPACE;
		let key = "FJCRXLUDN";
		let txt = "CODIGO".to_owned();
		let dim = (key.len() as f64).sqrt() as usize;
		let fill_char = 'L';
		let key_mtrx = key_mtrx_repr(&key, &dim, namespace);
		let parts = src_text_vec_repr(txt, &fill_char, &dim, namespace).unwrap();

		assert_eq!(
			cypher_src_vecs(parts, dim, &key_mtrx, namespace),
			vec![String::from("WLP"), String::from("GSE")]
		);
	}

	#[test]
	fn not_square_length_source_text_parts_are_turned_into_cyphered_parts() {
		let namespace = &DEFAULT_NAMESPACE;
		let key = "ABCDEFGHI";
		let txt = "TESTTEXTs".to_owned();
		let fill_char = 'L';
		let dim = (key.len() as f64).sqrt() as usize;
		let key_mtrx = key_mtrx_repr(&key, &dim, namespace);
		let parts = src_text_vec_repr(txt, &fill_char, &dim, namespace).unwrap();

		assert_eq!(
			cypher_src_vecs(parts, dim, &key_mtrx, namespace),
			vec![
				String::from(""), String::from(""), String::from(""),
			]
		);
	}
}
