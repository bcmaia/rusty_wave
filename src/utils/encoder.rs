use rayon::prelude::*;
use super::{
	bit::{Bit, Nibble},
	sample::Sample,
	pulse::{PulsePattern, Pulse, PULSE_SIZE, MID_INDEX},
	line_encoders::{
		LineEncoding, // type
		nrz, // function
		nrzi, // function
		bipolar_ami, // function
		pseudoternario, // function
		manchester, // function
		manchester_dif, // function
	},
	block_encoders::{
		BlockEncoding,
		be_4b5b,
		be_nop,
	},
};





pub struct Encoder {
	line_encoding : LineEncoding,
	block_encoding : Option<BlockEncoding>,
}



impl Encoder {
	pub fn new (block : u8, line : u8) -> Encoder {
		Encoder {
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
		// Breaking the bits in nibbles
		let nibbles = Nibble::to_nibbles(bits); 
		// Making sure we have no "Not Set" bit
		let nibbles = 
			nibbles.par_iter() // Using par_iter() to get a parallel iterator
			.map(|nibble| nibble.define()).collect::<Vec<_>>()
		;

		let enc = match self.block_encoding {
			Some(x) => x,
			None => be_nop, // Obs: should not happen, but if it does...
		};

		let encoded_bits = nibbles
			.into_par_iter()
			.map(|nibble| {enc(nibble)} )
			.collect::<Vec<_>>()
		;

		encoded_bits.into_iter().flatten().collect()
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
		let pulses = patterns
			.into_iter()
			.map( |p| {Pulse::from_pattern(&p)} )
			.collect::<Vec<_>>()
		;

		pulses
	}

	pub fn render (pulses : &Vec<Pulse>) -> Vec<String> {
		let mut samples = pulses
			.into_iter()
			.map( |p| {p.separate_samples()} )
			.into_iter()
			.flatten()
			.collect::<Vec<_>>()
		;

		samples.insert(0, Sample::NoSignal);

		let interpolated_samples = samples
			.windows(2)
			.map(|pair| {Sample::interpolate(&pair[0], &pair[1])})
			.collect::<Vec<_>>()
		;

		let empty_string = String::from("");
		let mut result = Vec::new();
		for (i, sample) in interpolated_samples.iter().enumerate() {
			let configs = match i % PULSE_SIZE {
				0 => (true, &empty_string,),
				MID_INDEX => (false, &pulses[i / PULSE_SIZE].title,),	// WARNING: HARD CODED
				_ => (false, &empty_string,),
			};

			result.push(sample.render(configs.0, configs.1));
		}

		result
	}
	
	
}
