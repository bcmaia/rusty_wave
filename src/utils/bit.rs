#[derive(Clone, Copy, PartialEq)]
pub enum Bit {
    Zero = 0,
    One = 1,
    NotSet = 2,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Nibble {
	pub data : [Bit; 4],
}

impl Bit {
    pub fn _toggle(self) -> Bit {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
            Bit::NotSet => Bit::NotSet,
        }
    }

	pub fn _to_bool (self) -> bool {
		match self {
			Bit::Zero => false,
			Bit::One => true,
			Bit::NotSet => false,
		}
	}

	pub fn char_to_bit (c : char) -> Bit {
		match c {
		    '0' => Bit::Zero,
			'1' => Bit::One,
	    	 _  => Bit::NotSet,
	    }
	}

    pub fn string_to_bits (s : &String) -> Vec<Bit> {
    	s.chars().map( |c| {Bit::char_to_bit(c)} ).collect::<Vec<Bit>>()
	}

	pub fn define (self) -> Bit {
		if Bit::NotSet == self {Bit::Zero}
		else {self}
	}
}



impl Nibble {
	pub fn define (self) -> Nibble {
		Nibble {
			data: [
				self.data[0].define(),
				self.data[1].define(),
				self.data[2].define(),
				self.data[3].define(),
			],
		}
	}

	pub fn to_nibbles(bits : &Vec<Bit>) -> Vec<Nibble> {
		let chunks = bits.chunks(4);
	    let mut nibbles = Vec::new();

	    for chunk in chunks {
	    	let mut nibble = Nibble{data: [Bit::NotSet; 4],};
	        for (i, bit) in chunk.iter().enumerate() {
	        	nibble.data[i] = *bit;
	        }
	        
	        nibbles.push(nibble);
	    }

	    nibbles
	}
}
