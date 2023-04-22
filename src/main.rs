use std::io;

mod bit;
mod sample;

use bit::Bit;
use sample::Sample;

fn main() {
	let a = [1, 3];
	let b = a[Bit::Zero as usize];
	println!("Bit size: {}", std::mem::size_of::<Bit>());

	// Read user input
	println!("Write a sequence of zeros and ones: ");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read input");

	let signals = [
        "┇     ┊      ", // 0
        "      ┊      ", // 1
        "      ┆     ┇", // 2
        "┅┅┅┅┅┅┊      ", // 3
        "      ┊┅┅┅┅┅┅", // 4
		"┅┅┅┅┅┅┅┅┅┅┅┅┅", // 5
	];

	let m : [[i32; 6]; 6] = [
      // 0, 1, 2, 3, 4, 5
		[0, 3, 5, 3, 4, 5], // 0
		[3, 1, 4, 3, 4, 5], // 1
		[5, 4, 2, 3, 4, 5], // 2
		[0, 1, 2, 3, 4, 5], // 3
		[0, 1, 2, 3, 4, 5], // 4
		[0, 1, 2, 3, 4, 5], // 5
	];
	
	// Iterate over input characters
	let mut last : i32 = 1;
	for c in input.chars() {
		let signal = get_signal(&c);
		let first = m[last as usize][signal[0] as usize];

		println!("  ┈┈{}┈┈", signals[first as usize]);
		println!("    {}", signals[signal[1] as usize]);
		println!("    {} {}", signals[signal[2] as usize], c);
		println!("    {}", signals[signal[3] as usize]);
		println!("    {}", signals[signal[4] as usize]);

		last = signal[5 - 1];
	}
}

fn get_signal(c : &char) -> [i32; 5] {
	match c {
		'0' => [2, 2, 5, 0, 0],
		'1' => [0, 0, 5, 2, 2],
		 _  => [1, 1, 1, 1, 1],
	}
}

    
