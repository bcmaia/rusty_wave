#[derive(Clone, Copy)]
pub enum Bit {
    Zero = 0,
    One = 1,
    NotSet = 2,
}


impl Bit {
    pub fn toggle(&self) -> Bit {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
            Bit::NotSet => Bit::NotSet,
        }
    }

    pub fn to_bool (&self) -> bool {
    	match self {
    		Bit::Zero => false,
    		Bit::One => true,
    		Bit::NotSet => false,
    	}
    }

	pub fn char_to_bit (c : &char) -> Bit {
		match c {
		    '0' => Bit::Zero,
			'1' => Bit::One,
	    	 _  => Bit::NotSet,
	    }
	}

    pub fn string_to_bits (s : &String) -> Vec<Bit> {
    	let mut v = Vec::<Bit>::new();
    	for c in s.chars() {
    		v.push(Bit::char_to_bit(&c));
        }
   
    	v
	}
}
