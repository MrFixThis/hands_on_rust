use std::io;
use cesar_cypher::App;


fn main() -> Result<(), io::Error>{
	let app = App::new();
	app.run()?;

	Ok(())
}
