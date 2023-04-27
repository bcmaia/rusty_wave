use super::{
    bit::Bit, 
    sample::Sample,
    pulse::{PulsePattern},
};

pub type LineEncoding = fn(&Bit, &mut Bit) -> PulsePattern;

pub fn nrz (
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

pub fn nrzi (
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

pub fn bipolar_ami (
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


pub fn pseudoternario (
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

pub fn manchester (
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

pub fn manchester_dif (
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

