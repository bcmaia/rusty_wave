mod sample;

use sample::Sample;

const PULSE_SIZE : usize = 5usize; 

pub struct PulsePattern {
	title : String,
	start : Sample,
	end : Sample,
}

pub struct Pulse {
	samples : [Sample, PULSE_SIZE],
}

impl Pulse {
	
}
