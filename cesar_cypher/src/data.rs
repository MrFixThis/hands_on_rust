use std::collections::HashMap;

use aho_corasick::AhoCorasick;

/// This array (in the future a Vector) contains the English's language
/// frequency analysis that will be used to associate its values to the ones
/// retrieved from the resulting frequency analysis over the encripted text
/// passed to the program
const LETTERS_FREQ: [(char, f64); 26] = [
	('E', 11.160), ('A', 8.4966), ('R', 7.5809), ('I', 7.5448), ('O', 7.1635),
	('T', 6.9509), ('N', 6.6544), ('S', 5.7351), ('L', 5.4893), ('C', 4.5388),
	('U', 3.6308), ('D', 3.3844), ('P', 3.1671), ('M', 3.0129), ('H', 3.0034),
	('G', 2.4705), ('B', 2.0720), ('F', 1.8121), ('Y', 1.7779), ('W', 1.2899),
	('K', 1.1016), ('V', 1.0074), ('X', 0.2902), ('Z', 0.2722), ('J', 0.1965),
	('Q', 0.1962)
];

/// This structs represents an analizer process over some cyphered text
/// passed to the program.
///
/// It's important to keep in mind that it is build over the context of the
/// problem in resolution, so you may only expect a valid result from this program,
/// if you are actually working with data that fits the context of the dilema.
pub struct Analizer {
	_eng_freq: Vec<(char, f64)>
}

impl Analizer {
	/// Creates a new `Analizer` instance.
	pub fn new() -> Self {
		Self {
			_eng_freq: Vec::from(LETTERS_FREQ)
		}
	}

	/// Retrieves the vector containing the `English`'s frequency analysis values.
	pub fn get_english_freqs(&self) -> &Vec<(char, f64)> {
		&self._eng_freq
	}

	/// Determines the frequency of each character inside a given text.
	pub fn calculate_frequency<'a>(
		&self,
		text_chars: &'a [char]
	) -> Vec<(&'a char, f64)>
	{
		let mut freq: HashMap<&'a char, f64> = HashMap::new();

		// here, we iterate over the given text to determine how many times
		// each character is inside it
		for c in text_chars.iter() { *freq.entry(c).or_insert(0.0) += 1.0; }

		// then, we calculate the frequency of each character with the formula:
		// f = c / l
		// where:
		// f = frequency
		// c = character
		// l = length of the text
		let text_len = text_chars.len();
		for (_, f) in freq.iter_mut() {
			*f = *f / text_len as f64;
		}

		// finally, the BTreeMap is converted to a vector sorted vector.
		let mut freq_vec: Vec<_> = freq.into_iter().collect();
		freq_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
		freq_vec.reverse();

		freq_vec
	}

	/// Associates the a given text's frequency with the frequency given by the
	/// `English`'s language frequency analysis.
	pub fn associate_frequency<'a, 'b>(
		&'a self,
		text: &str,
		freq: &'b [(&'a char, f64)]
	) -> Vec<(&'a char, &'b char)>
	{
		let mut eng_freq_iter = self._eng_freq.iter();
		let mut eng_freq_item = eng_freq_iter.next().unwrap();
		let mut prev_val = &freq[0].1;
		freq
			.into_iter()
			.map(|(c, f)| {
				match *f {
					n if n != 1.0 / text.len() as f64 => {
						if prev_val != f {
							eng_freq_item = eng_freq_iter.next().unwrap();
						}
						prev_val = f;
						(*c, &eng_freq_item.0)
					},
					_ => (*c, &'?')
				}
			})
		.collect()
	}

	/// Assembles a result by replacing, over the original text, all the
	/// corresponding characters from a previos association to the `English`'s
	/// frequency analysis.
	pub fn assemble_result(
		&self,
		text: &str,
		text_association: &[(&char, &char)]) -> String
	{
		let patterns: Vec<_> = text_association
			.iter()
			.map(|(&c, _)| c.to_string())
			.collect();

		let replacement: Vec<_> = text_association
			.iter()
			.map(|(_, &c)| c.to_string())
			.collect();

		let ac = AhoCorasick::new(&patterns);
		ac.replace_all(text, &replacement)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn english_freqs_are_retrieved() {
		let analizer: Analizer = Analizer::new();
		let eng_freqs = analizer.get_english_freqs();

		assert_eq!(&analizer._eng_freq, eng_freqs)
	}
	#[test]
	fn frequency_is_calculated() {
		let analizer: Analizer = Analizer::new();
		let text = String::from("TEQQEE");

		analizer.calculate_frequency(&text.chars().collect::<Vec<_>>());
	}

	#[test]
	fn frequency_is_associated() {
		let analizer: Analizer = Analizer::new();
		let text = String::from("AAACCENTM");
		let chars = &text.chars().collect::<Vec<_>>();
		let freq = analizer.calculate_frequency(chars);
	}

	#[test]
	fn result_is_propertly_assembled() {
		let analizer: Analizer = Analizer::new();
		let text = String::from("TEBKFKQEBZLROPBLCERJXKBSBKQP");
		let text_chars: Vec<_> = text.chars().collect();
		let freq = analizer.calculate_frequency(&text_chars);
		println!("{freq:?}");
		let ass_freqs = analizer.associate_frequency(&text, &freq);
		println!("{ass_freqs:?}");

		let res = analizer.assemble_result(&text, &ass_freqs);

		assert_eq!(res, String::from("ABC"))
	}
}
