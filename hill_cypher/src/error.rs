use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
	#[error("{0}")]
	ProcessingError(String),
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
		Error::ProcessingError(value.to_owned())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
		Error::ProcessingError(value)
    }
}

#[macro_export]
macro_rules! error_msg {
    ($msg:expr) => {
		let colorizer = colored::Colorize;
		eprint!("{}", colorizer::bold(colorizer::red("error")));
		eprint!("{}", colorizer::bold(": "));
		eprintln!("{}", colorizer::bold(&msg));
	};
}
