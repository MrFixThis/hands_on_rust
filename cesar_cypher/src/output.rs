use std::fmt::Display;

#[derive(Debug)]
pub struct Logger;

impl Logger {
    pub fn new() -> Self { Self }

	pub fn format_report(
		&self,
		text: &str,
		result: &str,
		eng_freq: &[(char, f64)],
		text_freq: &[(&char, f64)],
		text_ass: &[(&char, &char)]
	) -> String {
		let mut template = String::new();
		let eng_freq_fmt = Self::_format_pairs(eng_freq);
		let text_freq_fmt = Self::_format_pairs(text_freq);
		let text_ass_fmt = Self::_format_pairs(text_ass);
		let posibilities = Self::_format_multi_posibilities(result, eng_freq);

		template.push_str(&format!("
 +---------------------------+
<| [Results of the Analysis] |>
 +---------------------------+

@> English Frequency Analysis <@
+-------------+-------------+
|  Character  |  Frequency  |
+-------------+-------------+
{eng_freq_fmt}
+-------------+-------------+

@> Input Frequency Analysis <@
+-------------+-------------+
|  Character  |  Frequency  |
+-------------+-------------+
{text_freq_fmt}
+-------------+-------------+

@> Input Associations <@
+-------------+-------------+
|     From    |      To     |
+-------------+-------------+
{text_ass_fmt}
+-------------+-------------+

@> Original and Resulting Texts <@
[Original]: {text}
[Result]:   {result}
[Unknown characters posibilities]:
{posibilities}
	"));
		
		template
	}

	fn _format_pairs<T, U>(result: &[(T, U)]) -> String
		where
			T: Display,
			U: Display
	{
		result
			.iter()
			.map(|(c, f)| {
				format!("| {c:^11} | {f:^11.*} |", 4)
			})
			.collect::<Vec<String>>()
			.join("\n")
			.to_owned()
	}

	fn _format_multi_posibilities(text: &str, alphabet: &[(char, f64)]) -> String {
		let mut template = String::new();

		if !text.contains('?') {
			template.push_str("There is not unknown characters");
			return template;
		}

		alphabet
			.iter()
			.map(|(c, _)| c)
			.collect::<Vec<_>>()
			.iter()
			.map(|&c| {
				format!(
					"- {}\n",
					text.replace("?", &*c.to_string())
				)
			})
			.collect()
	}
}
