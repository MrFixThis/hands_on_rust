use std::io;
use cesar_encryptation::App;


fn main() -> Result<(), io::Error>{
	let app = App::new();
	app.run()?;

	Ok(())
}
