use structopt::StructOpt;
use structopt::clap::AppSettings;

/// Cypher and decypher text using the Hill's Cypher method
#[derive(Debug, StructOpt)]
#[structopt(
	name = "hill_cypher",
	author = "Bryan Baron <MrFixThis>",
	rename_all = "kebab-case",
	setting = AppSettings::DeriveDisplayOrder,
	setting = AppSettings::ColoredHelp,
)]
pub struct Args {
	#[structopt(subcommand)]
	pub cmd: Command,
}

// This struct represents the application's available commands
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum Command {
	/// Cypher a given source text
	Cypher {
		/// Key to cypher the source text
		#[structopt(short, long)]
		key: String,

		/// Source text to cypher
		#[structopt(short, long)]
		source: String,

		/// Dimension of the key's representation matrix
		#[structopt(short, long, required_unless("namespace"))]
		dimension: Option<usize>,

		/// Key's and source text's fill letter for the case where
		/// dimension^2 != source's length
		#[structopt(short, long)]
		fill_letter: char,

		/// Custom namespace for the base of the algorithm
		#[structopt(short, long)]
		namespace: Option<String>,
	},

	/// Decypher a given source text
	Decypher {
		/// Key to decypher the source text
		#[structopt(short, long)]
		key: String,

		/// Source text to decypher
		#[structopt(short, long)]
		source: String,

		/// Known key's and source text's fill letter
		#[structopt(short, long)]
		fill_letter: Option<char>,

		/// Known namespace used to cypher the source cyphered text
		#[structopt(short, long)]
		namespace: Option<String>
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;
//
// 	#[test]
// 	fn arg_parser_takes_input() {
// 		let t_args = vec![
// 			// "hill_cypher", // program's name
// 			// "cypher",
// 			// "-h"
// 			// "-k my_test_key",
// 			// "-s my_test_source_text",
// 		];
//
// 		let args = Args::from_iter(t_args.iter());
// 		println!("{args:?}");
// 	}
// }
