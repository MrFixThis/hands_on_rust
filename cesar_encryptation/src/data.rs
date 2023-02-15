use std::collections::BTreeMap;

/// This array (in the future a HashMap) contains the English's language
/// frequency analysis that will be used to associed its values to the ones
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

///
pub struct Analizer {
	_eng_freqs: Vec<(char, f64)>
}

impl Analizer {
	/// Createso a new `Analizer` instance
	pub fn new() -> Self {
		Self {
			_eng_freqs: Vec::from(LETTERS_FREQ)
		}
	}

	pub fn get_english_freqs(&self) -> &Vec<(char, f64)> {
		&self._eng_freqs
	}

	pub fn calculate_frequency<'a>(&self, text: &'a[char]) -> BTreeMap<&'a char, f64> {
		let mut freq: BTreeMap<&'a char, f64> = BTreeMap::new();

		// here, we iterate over the given text to determine how many times
		// each character is inside it
		for c in text {
			let n = freq.entry(c).or_insert(0.0);
			*n += 1.0;
		}

		// then, we calculate the frequency of each character with the formula
		// f = c / l
		// where:
		// f = frequency
		// c = character
		// l = length of the text
		let text_len = text.len();
		for (_, f) in freq.iter_mut() {
			*f = *f / text_len as f64;
		}

		freq
	}

	pub fn associate_frequency<'a, 'b>(
		&'a self,
		freq: &'b BTreeMap<&'a char, f64>
	) -> BTreeMap<&'a char, &'b char> {
		// here, we are sorting the values of the frequencies HashMap by value
		// to later associate its values with the english's frequency table
		let mut freq_vec: Vec<_> = freq.into_iter().collect();
		freq_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
		freq_vec.reverse();

		let freq: BTreeMap<_, _> = freq_vec.into_iter().collect();

		freq
			.into_iter()
			.enumerate()
			.map(|(i, (k, _))| {
				(*k, &self._eng_freqs[i].0)
			}).
			collect()
	}

	pub fn assemble_result(&self, parts: &[&char]) -> String {
		let mut result = String::with_capacity(parts.len());
		for c in parts {
			result.push(**c)
		}
		result
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn english_freqs_are_retrieved() {
		let analizer: Analizer = Analizer::new();
		let eng_freqs = analizer.get_english_freqs();

		assert_eq!(&analizer._eng_freqs, eng_freqs)
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

		let chars = &text.chars().collect::<Vec<_>>();
		let freq = analizer.calculate_frequency(chars);
		let ass_freqs = analizer.associate_frequency(&freq);

		let vals: Vec<_> = ass_freqs.into_values().collect();
		let res = analizer.assemble_result(&vals);

		assert_eq!(res, String::from("ABC"))
	}
}
