mod data;
mod report;

use std::{env, io::{self, ErrorKind}};

use data::Analizer;
use report::Logger;

pub struct App {
	analizer: Analizer,
	logger: Logger
}

impl App {
	pub fn new() -> Self {
		Self {
			analizer: Analizer::new(),
			logger: Logger::new()
		}
	}

	/// Runs the application and performs the analysis with the given argument
	pub fn run(&self) -> Result<(), io::Error> {
		let arg = Self::get_arg()?;
		let eng_freq = self.analizer.get_english_freqs();
		let arg_chars: Vec<_> = arg.chars().collect();
		let arg_freq = self.analizer.calculate_frequency(&arg_chars);
		let arg_ass = self.analizer.associate_frequency(&arg, &arg_freq);
		let result = self.analizer.assemble_result(&arg, &arg_ass);

		println!("{}",
			self.logger.format_report(
				&arg,
				&result,
				&eng_freq,
				&arg_freq,
				&arg_ass
			)
		);

		Ok(())
	}

	fn get_arg() -> Result<String, io::Error> {
		// using args_os to avoid a possible panic
		let cmd_args: Vec<_> = env::args_os().collect();
		// ensuring that the program got an argument
		match cmd_args.len() {
			2  => {
				// getting the input argument into the form required by the program
				Ok(cmd_args
				   .get(1)
				   .unwrap()
				   .to_string_lossy()
				   .trim()
				   .to_uppercase()
				   .to_string()
				)
			},
			n => {
				if n == 1 {
					Err(
						io::Error::new(
							ErrorKind::InvalidData,
							"missing argument. specify any valid value"
						)
					)
				} else {
					Err(
						io::Error::new(
							ErrorKind::InvalidInput,
							"too many arguments. check your input data"
						)
					)
				}
			}
		}
	}
}
