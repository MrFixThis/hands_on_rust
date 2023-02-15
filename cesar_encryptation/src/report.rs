use std::collections::HashMap;

pub struct Logger;

impl Logger {
    pub fn new() -> Self { Self }

	// pub fn format_report() -> String {
	//
	// }

	fn format_freqs(freq: &HashMap<&char, f64>) -> String {
		let mut report = String::with_capacity(freq.len());

		for (c, f) in freq {
			report.push_str("-> Letter: {c}\n   Frequency: {.4:f}%");
		}

		report
	}
}
