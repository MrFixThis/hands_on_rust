mod data;
mod report;

use std::{env, io::{self, ErrorKind}};

use data::Analizer;
use report::Logger;

pub struct App {
	analizer: Analizer,
	reporter: Logger
}

impl App {
	pub fn new() -> Self {
		Self {
			analizer: Analizer::new(),
			reporter: Logger::new()
		}
	}

	// pub fn run() -> Result<(), io::Error> {
	// 	let arg = Self::get_arg()?;
	//
	// }

	fn get_arg() -> Result<String, io::Error> {
		// using args_os to avoid a possible panic
		let cmd_args: Vec<_> = env::args_os().collect();

		// ensuring that the program got an argument
		match cmd_args.len() {
			n if n < 1 => {
				Err(
					io::Error::new(
						ErrorKind::InvalidData,
						"missing argument. specify any valid value"
					)
				)
			},
			n if n > 2 => {
				Err(
					io::Error::new(
						ErrorKind::InvalidInput,
						"too many arguments. check your input data"
					)
				)
			},
			_ => {
				// getting the input argument into the form required by the program
				Ok(cmd_args
				   .get(1)
				   .unwrap()
				   .to_string_lossy()
				   .trim()
				   .to_uppercase()
				   .to_string()
				)
			}
		}
	}
}
