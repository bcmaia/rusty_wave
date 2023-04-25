mod bit;
mod sample;
mod pulse;

use bit::Bit;
use sample::Sample;
use pulse::{PulsePattern, Pulse};


type LineEncoding = fn(Bit, &mut Bit) -> PulsePattern;
type BlockEncoding = fn(Nibble) -> Vec::<Bit>;

fn nrz (
	b : Bit,
	state : &mut Bit
) -> PulsePattern {
	match b {
		Bit::Zero => PulsePatern{
			title: String::from("0"),
			start: Sample::Low,
			end: Sample::Low,
		},
		Bit::One => PulsePatern{
			title: String::from("1"),
			start: Sample::High,
			end: Sample::High,
		},
		_ => PulsePatern{
         	title: String::from("0"),
		 	start: Sample::Low,
		 	end: Sample::Low,
		},
	}
}

pub struct Encoding {
	line_encoding : LineEncoding,
	block_encoding : Option<BlockEncoding>,
}

pub fn BE_4B5B (nibble: Nibble) -> Vec<Bit> {
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
    let code = table[nibble[0] as usize * 8 + nibble[1] as usize * 4 + nibble[2] as usize * 2 + nibble[3] as usize];

    // Add extra zero bit for DC balance
    let mut bits = Vec::<Bit>::new();//vec![Bit::Zero];
    bits.extend_from_slice(&code);
    bits
}

pub fn BE_NOP (nibble : Nibble) -> Vec::<Bit> {
	let mut bits = Vec::<Bit>::new();
	bits.extend_from_slice(&nibble);
	bits
}



impl Encoding {
	pub fn new (block : u8, line : u8) {
		Encoding {
			line_encoding: match line {
				1 => nrz,
				_ => nrz,
			},
			block_encoding: match block {
				1 => BE_NOP,
				2 => BE_4B5B,
				_ => BE_NOP,
			},
		}
	}
	
	pub fn block_encode (&self, bits : &Vec::<Bit>) {
		let nibbles = Nibbles::to_nibbles(bits);
		let mut encoded_bits = Vec::new();

		let enc = match self.block_encode {
			
		}
		
		for nibble in nibbles {
		    let encoded_block = self.block_encoding(nibble);
			encoded_bits.append(&mut encoded_block);
		}
		
		encoded_bits
	}

	pub fn line_encode (&self, bits : &Vec::<Bit>) -> Vec<PulsePattern> {
		let mut patterns = Vec::new();
		let mut state = Bit::Zero;
		
		
		for b in bits {
			patterns.push(self.line_encode(b, &mut state));
		}

		patterns
	}

	
}
