// mod bit;
// mod sample;
// mod pulse;
// mod encoding;

use std::io::{self};
mod utils;
use utils::encoder::Encoder;

fn main() {
	// Read user input
	println!("\n\nBlock Encoding method:  \n0 > No block encoding (default)  \n1 > 4B5B");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read input");
	let block_encode_index = match input.trim().parse::<i32>() {
		Ok(x) => x,
		Err(_) => {println!("ERROR: INVALID INPUT, ASSUMING DEFAULT AS ANSWER.\n\n"); 0}
	};

	println!("\n\nLine Encoding method:  \n0 > NRZ (default) \n1 > NRZI \n2 > Bipolar-AMI \n3 > Pseudoternário \n4 > Manchester \n5 > Manchester Diferencial");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read input");
	let line_encode_index = match input.trim().parse::<i32>() {
		Ok(x) => x,
		Err(_) => {println!("ERROR: INVALID INPUT, ASSUMING DEFAULT AS ANSWER.\n\n"); 0}
	};

	println!("\n\nWrite a sequence of zeros and ones: ");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read input");

	// Creating encoding pipeline
	let encod = Encoder::new(block_encode_index as u8, line_encode_index as u8);
	let text = String::from(input.trim());

	let signal = encod.encode(&text);
	println!("\n");
	Encoder::print(&signal);
	println!("\n");
}

    
