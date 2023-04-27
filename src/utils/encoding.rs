// mod bit;
// mod sample;
// mod pulse;

use super::bit::{Bit, Nibble};
use super::sample::Sample;
use super::pulse::{PulsePattern, Pulse, PULSE_SIZE, MID_INDEX};


type LineEncoding = fn(&Bit, &mut Bit) -> PulsePattern;
type BlockEncoding = fn(Nibble) -> Vec::<Bit>;

fn nrz (
	b : &Bit,
	_state : &mut Bit
) -> PulsePattern {
	match b {
		Bit::Zero => PulsePattern{
			title: String::from("0"),
			start: Sample::Low,
			end: Sample::Low,
		},
		Bit::One => PulsePattern{
			title: String::from("1"),
			start: Sample::High,
			end: Sample::High,
		},
		_ => PulsePattern{
         	title: String::from("No Signal"),
		 	start: Sample::NoSignal,
		 	end: Sample::NoSignal,
		},
	}
}

fn nrzi (
	b : &Bit,
	last_bit : &mut Bit
) -> PulsePattern {

	let result = match b {
		Bit::Zero => if Bit::Zero == *last_bit {
			PulsePattern{
				title: String::from("0"),
				start: Sample::Low,
				end: Sample::Low,
			}
		} else {
			PulsePattern{
				title: String::from("0"),
				start: Sample::High,
				end: Sample::High,
			}
		},
		Bit::One => if Bit::One == *last_bit {
			PulsePattern{
				title: String::from("1"),
				start: Sample::Low,
				end: Sample::Low,
			}
		} else {
			PulsePattern{
				title: String::from("1"),
				start: Sample::High,
				end: Sample::High,
			}
		},
		_ => PulsePattern{
         	title: String::from("No Signal"),
		 	start: Sample::NoSignal,
		 	end: Sample::NoSignal,
		},
	};

	*last_bit = if Sample::High == result.start {Bit::One} else {Bit::Zero};

	result
}

fn bipolar_ami (
	b : &Bit,
	last_bit : &mut Bit
) -> PulsePattern {

	let result = match b {
		Bit::Zero => PulsePattern{
			title: String::from("0"),
			start: Sample::Mid,
			end: Sample::Mid,
		},
		Bit::One => if Bit::One == *last_bit {
			PulsePattern{
				title: String::from("1"),
				start: Sample::Low,
				end: Sample::Low,
			}
		} else {
			PulsePattern{
				title: String::from("1"),
				start: Sample::High,
				end: Sample::High,
			}
		},
		_ => PulsePattern{
         	title: String::from("No Signal"),
		 	start: Sample::NoSignal,
		 	end: Sample::NoSignal,
		},
	};

	if Sample::High == result.start {*last_bit = Bit::One} 
	else if Sample::Low == result.start {*last_bit = Bit::Zero}

	result
}


fn pseudoternario (
	b : &Bit,
	last_bit : &mut Bit
) -> PulsePattern {

	let result = match b {
		Bit::One => PulsePattern{
			title: String::from("1"),
			start: Sample::Mid,
			end: Sample::Mid,
		},
		Bit::Zero => if Bit::One == *last_bit {
			PulsePattern{
				title: String::from("0"),
				start: Sample::Low,
				end: Sample::Low,
			}
		} else {
			PulsePattern{
				title: String::from("0"),
				start: Sample::High,
				end: Sample::High,
			}
		},
		_ => PulsePattern{
         	title: String::from("No Signal"),
		 	start: Sample::NoSignal,
		 	end: Sample::NoSignal,
		},
	};

	if Sample::High == result.start {*last_bit = Bit::One} 
	else if Sample::Low == result.start {*last_bit = Bit::Zero}

	result
}

fn manchester (
	b : &Bit,
	_state : &mut Bit
) -> PulsePattern {
	match b {
		Bit::Zero => PulsePattern{
			title: String::from("0"),
			start: Sample::High,
			end: Sample::Low,
		},
		Bit::One => PulsePattern{
			title: String::from("1"),
			start: Sample::Low,
			end: Sample::High,
		},
		_ => PulsePattern{
         	title: String::from("No Signal"),
		 	start: Sample::NoSignal,
		 	end: Sample::NoSignal,
		},
	}
}

fn manchester_dif (
	b : &Bit,
	last_bit : &mut Bit
) -> PulsePattern {

	let result = match b {
		Bit::Zero => if Bit::One == *last_bit {
			PulsePattern{
				title: String::from("0"),
				start: Sample::High,
				end: Sample::Low,
			}
			
		} else {
			PulsePattern{
				title: String::from("0"),
				start: Sample::Low,
				end: Sample::High,
			}
		},
		Bit::One => if Bit::Zero == *last_bit {
			PulsePattern{
				title: String::from("1"),
				start: Sample::High,
				end: Sample::Low,
			}
		} else {
			PulsePattern{
				title: String::from("1"),
				start: Sample::Low,
				end: Sample::High,
			}
		},
		_ => PulsePattern{
         	title: String::from("No Signal"),
		 	start: Sample::NoSignal,
		 	end: Sample::NoSignal,
		},
	};

	*last_bit = if Sample::High == result.end {Bit::One} else {Bit::Zero};

	result
}



pub struct Encoding {
	line_encoding : LineEncoding,
	block_encoding : Option<BlockEncoding>,
}

pub fn be_4b5b (nibble: Nibble) -> Vec<Bit> {
    // Lookup table for 4B5B block encoding
    let table: [[Bit; 5]; 16] = [
        [Bit::One, Bit::Zero, Bit::Zero, Bit::One, Bit::One],  // 0000
        [Bit::Zero, Bit::One, Bit::One, Bit::Zero, Bit::Zero],  // 0001
        [Bit::Zero, Bit::One, Bit::Zero, Bit::One, Bit::One],  // 0010
        [Bit::Zero, Bit::One, Bit::Zero, Bit::Zero, Bit::One],  // 0011
        [Bit::One, Bit::Zero, Bit::One, Bit::One, Bit::Zero],  // 0100
        [Bit::One, Bit::Zero, Bit::One, Bit::Zero, Bit::Zero],  // 0101
        [Bit::One, Bit::Zero, Bit::Zero, Bit::Zero, Bit::One],  // 0110
        [Bit::One, Bit::Zero, Bit::Zero, Bit::One, Bit::Zero],  // 0111
        [Bit::Zero, Bit::One, Bit::One, Bit::Zero, Bit::One],  // 1000
        [Bit::Zero, Bit::One, Bit::One, Bit::One, Bit::Zero],  // 1001
        [Bit::Zero, Bit::One, Bit::Zero, Bit::One, Bit::Zero],  // 1010
        [Bit::One, Bit::One, Bit::Zero, Bit::Zero, Bit::One],  // 1011
        [Bit::One, Bit::One, Bit::Zero, Bit::One, Bit::Zero],  // 1100
        [Bit::One, Bit::One, Bit::One, Bit::Zero, Bit::Zero],  // 1101
        [Bit::One, Bit::One, Bit::One, Bit::Zero, Bit::One],  // 1110
        [Bit::One, Bit::One, Bit::One, Bit::One, Bit::Zero],  // 1111
    ];

    // Convert nibble to 5-bit code using lookup table
    let code = table[nibble.data[0] as usize * 8 + nibble.data[1] as usize * 4 + nibble.data[2] as usize * 2 + nibble.data[3] as usize];

    // Add extra zero bit for DC balance
    let mut bits = Vec::<Bit>::new();//vec![Bit::Zero];
    bits.extend_from_slice(&code);
    bits
}

pub fn be_nop (nibble : Nibble) -> Vec::<Bit> {
	let mut bits = Vec::<Bit>::new();
	bits.extend_from_slice(&nibble.data);
	bits
}



impl Encoding {
	pub fn new (block : u8, line : u8) -> Encoding {
		Encoding {
			line_encoding: match line {
				0 => nrz,
				1 => nrzi,
				2 => bipolar_ami,
				3 => pseudoternario,
				4 => manchester,
				5 => manchester_dif,
				_ => nrz,
			},
			block_encoding: match block {
				0 => None,
				1 => Some(be_4b5b),
				_ => None,
			},
		}
	}
	
	pub fn block_encode (&self, bits : &Vec::<Bit>) -> Vec<Bit> {
		let nibbles = Nibble::to_nibbles(bits);
		let mut encoded_bits = Vec::new();

		let enc = match self.block_encoding {
			Some(x) => x,
			None => be_nop, // Obs: should not happen, but if it does...
		};
		
		for nibble in nibbles {
		    let mut encoded_block = enc(nibble);
			encoded_bits.append(&mut encoded_block);
		}
		
		encoded_bits
	}

	pub fn line_encode (&self, bits : &Vec::<Bit>) -> Vec<PulsePattern> {
		let mut patterns = Vec::new();
		let mut state = Bit::Zero;
		let enc = self.line_encoding;
		
		for b in bits {
			patterns.push(enc(b, &mut state));
		}

		patterns
	}

	pub fn encode (&self, input : &String) -> Vec<Pulse> {
		let bits = Bit::string_to_bits(input);
		let bits = match self.block_encoding {
			Some(_) => self.block_encode(&bits),
			None => bits,
		};

		let patterns = self.line_encode(&bits);
		let mut pulses = Vec::new();
		
		for p in patterns.into_iter() {
			pulses.push(Pulse::from_pattern(&p));
		}

		pulses
	}

	pub fn print (pulses : &Vec<Pulse>) {
		//let last_index = PULSE_SIZE - 1;
		//let mid_index = ((PULSE_SIZE as f32) / 2f32).ceil() as usize;

		let mut samples = Vec::new();

		for p in pulses.into_iter() {
			samples.append(&mut p.separate_samples());
		}

		let mut interpolated_samples = Vec::new();
		samples.insert(0, Sample::NoSignal);

		for pair in samples.windows(2) {
			interpolated_samples.push(Sample::interpolate(&pair[0], &pair[1]));
		}
		//interpolated_samples.remove(0);
		let samples = interpolated_samples;

		let empty_string = String::from("");
		for (i, sample) in samples.iter().enumerate() {
			let configs = match i % PULSE_SIZE {
				0 => (true, &empty_string,),
				MID_INDEX => (false, &pulses[i / PULSE_SIZE].title,),	// WARNING: HARD CODED
				_ => (false, &empty_string,),
			};

			sample.print(configs.0, configs.1);
		}
	}

	
}
