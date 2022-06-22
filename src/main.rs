use std::fs::File;
use std::io::Read;

use clap::Parser;

mod instruction;
mod value;
mod local;
mod chunk;
mod function;

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
	#[clap(parse(from_os_str))]
	file: std::path::PathBuf,
}


fn main() -> Result<(), std::io::Error> {
	let args = Args::parse();
	let mut input = File::open(args.file)?;
	let mut buffer = vec![0; input.metadata()?.len() as usize];

	input.read_exact(&mut buffer)?;

	println!("{:#?}", chunk::Chunk::parse(&buffer));

	Ok(())
}
