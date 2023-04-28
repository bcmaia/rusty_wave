use super::{
    bit::{Bit, Nibble}, 
};


pub type BlockEncoding = fn(Nibble) -> Vec::<Bit>;

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