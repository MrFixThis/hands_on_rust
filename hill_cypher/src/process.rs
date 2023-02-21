use derive_builder::Builder;
use rulinalg::matrix::{Matrix, BaseMatrix};

use crate::error::Result;

/// This constant array refers to the default namespace used by the `cypher` and
/// `decypher` algorithms to do its work. This value is obscured if a `custom
/// namespace` is specified.
pub const DEFAULT_NAMESPACE: [char; 27] = [
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
	'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ' '
];

/// A `Cypher` and `Decypher` processor.
///
/// This struct exposes the application's cypher and decypher capabilities
/// based on the `Hill's Method` cypher.
#[derive(Debug, Default, Builder)]
pub struct Processor<'a> {
	key: &'a str,
	source: &'a str,
	fill_letter: Option<&'a char>,
	name_space: Option<&'a str>,
}

impl Processor<'_> {
	/// Cyphers the given `source text` based on the information passed
	/// to the program, like a `key`, a `fill letter` or a `custom namespace`.
	pub fn cypher(&self) -> Result<String> {
		// definition of which namespace to use: either the user supplied
		// namespace or the default one
		let namespace: Vec<_> = match self.name_space {
			Some(ns) => {
				let ns_len = ns.len();
				if ns_len < 27 || !is_square(ns.len()) {
					return Err(
						format!(
							"the suplied namespace has to have a square length >= 27",
						).into()
					);
				}
				ns.chars().map(|c| c.to_ascii_uppercase()).collect()
			},
			None => DEFAULT_NAMESPACE.into_iter().collect()
		};

		// Checking the validness of the user suplied info
		chek_information(self.key, self.source, self.fill_letter, &namespace)?;

		// cheking if the key's length is square. If it is not, the
		// key is filled
		let key = if !is_square(self.key.len()) {
			fill_key(self.key, self.fill_letter.unwrap())?.to_uppercase()
		} else {
			self.key.to_uppercase()
		};

		// getting the key's length
		let dimension = (key.len() as f64).sqrt() as usize;

		// cheking if the source text's length is divisible by the dimension.
		// If it is not, the text is filled
		let source = if !is_divisble(self.source.len(), &dimension) {
			fill_src(self.source, self.fill_letter.unwrap(), &dimension)?
		} else {
			self.source.to_uppercase()
		};

		// getting the key's matrix representation and its determinant
		let key_mtrx = txt_mtrx_repr(dimension, dimension, &key, &namespace)?;
		let key_det = key_mtrx.clone().det(); // it is clone because det() consumes
											  // the the receiver

		// checking if the supplied key is valid to use for the cypher process
		if key_det == 0.0 || has_any_factor(
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
		// the key's matrix representation, and turning its values
		// into its respective numeric representation inside the namespace
		let src_mtrx = txt_mtrx_repr(
			source.len() / dimension,
			dimension,
			&source, &namespace)?;

		// turning the cyphered_parts into its textual representation
		Ok(cypher_src_mtrx(
				&key_mtrx,
				src_mtrx,
				&namespace
			)
		)
	}
}

// pub(crate) fn get_inverse_matrix<T>(source: &Matrix<T>) -> Matrix<T>
// where
// 	T: Ord + PartialOrd
// {
//
// }

/// Turns a given (Matrix)[rulinalg::matrix::Matrix] filled with the positions
/// of each character of a `source text`, into its textual representations.
fn cypher_src_mtrx(
	key_mtrx: &Matrix<f64>,
	src_mtrx: Matrix<f64>,
	namespace: &[char]
) -> String {
	// cyphering the source text's matrix
	let mtrx_mul = (key_mtrx * src_mtrx).transpose();
	mtrx_mul
		.into_vec()
		.into_iter()
		.map(|v| namespace[(v % (namespace.len() - 1) as f64) as usize])
		.collect()
}

/// Splits the given `source text` into as many parts as the given `dimension`
/// and turns its values into its respective representation inside the
/// namespace specified, and stores it inside a
/// (Matrix)[rulinalg::matrix::Matrix].
fn txt_mtrx_repr(
	width: usize,
	height: usize,
	src: &str,
	namespace: &[char]
) -> Result<Matrix<f64>>
{
	let parts: Vec<_> = src
		.chars()
		.map(|c| char_pos(&c, namespace) as f64)
		.collect();

	Ok(Matrix::new(width, height, parts).transpose())
}

/// Fills a given `key` that has not a square length with a specified
/// character that is present in the namespace.
fn fill_key(key: &str, char: &char) -> Result<String> {
	let key_len = key.len();
	Ok(fill_text(key, char, turn_perfect_sqrt(key_len), key_len))
}

/// Fills a given `source text` that has not a divisble length by
/// the `keys`'s `dimension` with a specified character that is present
/// in the namespace.
fn fill_src(src: &str, char: &char, key_dim: &usize) -> Result<String> {
	let src_len = src.len();
	Ok(fill_text(src, char, turn_divisible(src_len, key_dim), src_len))
}

/// Fills a given `text` with a specified character (a - b) times.
fn fill_text(txt: &str, char: &char, a: usize, b: usize) -> String {
	let reps = a - b;

	if reps != 0 {
		let append = char.to_string().repeat(reps);
		format!("{}{}", txt, append).to_uppercase()
	} else {
		txt.to_owned()
	}
}

/// Cheks the validness of the user supplied information. If something went
/// wrong in the cheking, (ProcessingError)[Error::ProcessingError] is returned.
fn chek_information(
	key: &str,
	src: &str,
	fill: Option<&char>,
	namespace: &[char]
) -> Result<()>
{
	// cheking if the supplied fill character is inside the namespace
	if let Some(f) = fill {
		is_in_namespace(f, &namespace)?;
	}

	// cheking if the supplied key and source text have an unkwnon character
	let mut target = key;
	for _ in 0..2 {
		for c in target.chars() {
			is_in_namespace(&c, &namespace)?;
		}
		target = src;
	}

	Ok(())
}

/// Checks if the supplied `character` is inside the given namespace; if it
/// is not, (ProcessingError)[Error::ProcessingError] is returned.
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

/// Retrives the given character's `position` inside the namespace specified.
fn char_pos(char: &char, namespace: &[char]) -> usize {
	namespace.iter().position(|&c| c == char.to_ascii_uppercase()).unwrap()
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

/// Checks if the supplied number is square.
fn is_square(num: usize) -> bool {
	let sqrt = (num as f64).sqrt();

	num == 0 || num == 1 || (sqrt * sqrt == num as f64)
}

/// Checks if the supplied target number is divisible by another one.
fn is_divisble(target: usize, num: &usize) -> bool {
	target % num == 0
}

/// Turns a given number a perfect square upwards it own value.
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

/// Turns a given target number divisible by another one.
fn turn_divisible(target: usize, dim: &usize) -> usize {
	let mut base = target;
	loop {
		if base % dim == 0 {
			return base;
		}
		base += 1;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn information_is_checked_as_valid() {
		let key = "TUNA";
		let src = "CODEINSIDE";
		let fill = Some(&'L');
		let namespace = &DEFAULT_NAMESPACE;

		assert!(!chek_information(&key, &src, fill, namespace).is_err());
	}

	#[test]
	fn information_is_checked_as_invalid() {
		let key = "TUNA232323";
		let src = "@CODEINSIDE.";
		let fill = Some(&'L');
		let namespace = &DEFAULT_NAMESPACE;

		assert!(chek_information(&key, &src, fill, namespace).is_err());
	}

	#[test]
	fn key_with_not_square_length_is_filled() {
		let src = "ABCDE".to_owned();
		let fill_char = 'L';

		assert_eq!(
			fill_key(&src, &fill_char).unwrap(),
			"ABCDELLLL"
		);
	}

	#[test]
	fn text_with_not_divisible_length_is_filled() {
		let key = "ABCDEFGHI";
		let key_dim = (key.len() as f64).sqrt() as usize;
		let src = "ABCD".to_owned();
		let fill_char = 'E';

		assert_eq!(fill_src(&src, &fill_char, &key_dim).unwrap(), "ABCDEE");
	}

	#[test]
	fn key_is_turned_into_matrix_representation() {
		let key = "ABCDEFGHI";
		let dim = (key.len() as f64).sqrt() as usize;

		assert_eq!(
			txt_mtrx_repr(dim, dim, &key, &DEFAULT_NAMESPACE).unwrap(),
			Matrix::new(dim, dim,
						vec![0.0, 3.0, 6.0,
							 1.0, 4.0, 7.0,
							 2.0, 5.0, 8.0]
						 )
		);
	}

	#[test]
	fn source_text_is_turned_into_mtrx_repr() {
		let _key = "FJCRXLUDN";
		let src = "CODIGO".to_owned();
		let dim = (_key.len() as f64).sqrt() as usize;

		assert_eq!(
			txt_mtrx_repr(src.len()/dim, dim, &src, &DEFAULT_NAMESPACE).unwrap(),
			Matrix::new(dim, src.len()/dim,
						vec![2.0, 8.0,
							 14.0, 6.0,
							 3.0, 14.0]
						 )
		);
	}

	#[test]
	fn source_text_parts_are_turned_into_cyphered_parts() {
		let namespace = &DEFAULT_NAMESPACE;
		let key = "FJCRXLUDN";
		let src = "CODIGO".to_owned();
		let dim = (key.len() as f64).sqrt() as usize;
		let key_mtrx = txt_mtrx_repr(dim, dim, &key, namespace).unwrap();
		let src_mtrx = txt_mtrx_repr(src.len()/dim, dim, &src, namespace).unwrap();

		assert_eq!(
			cypher_src_mtrx(&key_mtrx, src_mtrx, namespace),
			String::from("WLPGSE")
		);
	}
}
