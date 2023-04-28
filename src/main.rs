use std::io::{self};
mod utils;
use utils::encoder::Encoder;
use utils::wave_render::vertical_print;
use utils::pulse::PULSE_SIZE;

fn main() {
	let mut message = String::new();

	loop {
		// Read user input
		println!("\nDisplay method: \n0 > Vertical (default) \n1 > Horizontal");
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read input");
		let display_method = match input.trim().parse::<i32>() {
			Ok(x) => x,
			Err(_) => {println!("ERROR: INVALID INPUT, ASSUMING DEFAULT AS ANSWER.\n\n"); 0}
		};

		println!("\nBlock Encoding method:  \n0 > No block encoding (default)  \n1 > 4B5B");
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read input");
		let block_encode_index = match input.trim().parse::<i32>() {
			Ok(x) => x,
			Err(_) => {println!("ERROR: INVALID INPUT, ASSUMING DEFAULT AS ANSWER.\n\n"); 0}
		};

		println!("\nLine Encoding method:  \n0 > NRZ (default) \n1 > NRZI \n2 > Bipolar-AMI \n3 > Pseudoternário \n4 > Manchester \n5 > Manchester Diferencial");
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read input");
		let line_encode_index = match input.trim().parse::<i32>() {
			Ok(x) => x,
			Err(_) => {println!("ERROR: INVALID INPUT, ASSUMING DEFAULT AS ANSWER.\n\n"); 0}
		};

		let mut position = 0;

		'inner_loop: loop {
			if 0 == message.len() {
				println!("\nWrite a sequence of zeros and ones: ");
				io::stdin().read_line(&mut message).expect("Failed to read input");
			}

			// Creating encoding pipeline
			let encod = Encoder::new(block_encode_index as u8, line_encode_index as u8);
			let text = String::from(message.trim());

			let signal = encod.encode(&text);
			let render = Encoder::render(&signal);

			match display_method {
				1=> {vertical_print(&render, position)},
				_ => {
					let final_render = render.join("\n");
					println!("\n{}\n", final_render);
				},
			};
		
			'menu_loop: loop {
				let extra_msg = ["", "\n3 > Move left \n4 > Move right"];
				println!(
					"\nWhat to do now? \n0 > New Signal (Default) \n1 > Exit \n2 > Change encoder configs {}", 
					extra_msg[if 1 == display_method {1} else {0}],
				);
				
				let mut input = String::new();
				io::stdin().read_line(&mut input).expect("Failed to read input");
				let choice = match input.trim().parse::<i32>() {
					Ok(x) => x,
					Err(_) => {println!("ERROR: INVALID INPUT, ASSUMING DEFAULT AS ANSWER.\n\n"); 0}
				};

				match choice {
					0 => {message = String::new(); break 'menu_loop;},
					1 => return,
					2 => break 'inner_loop,
					3 => {position -= PULSE_SIZE as i32; vertical_print(&render, position); ()},
					4 => {position += PULSE_SIZE as i32; vertical_print(&render, position); ()},
					_ => (),
				}
			}
		}
	}
}

    
