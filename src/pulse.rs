mod sample;

use sample::Sample;

const PULSE_SIZE : usize = 5usize; 

pub struct PulsePattern {
	title : String,
	start : Sample,
	end : Sample,
}

pub struct Pulse {
	title: String,
	samples : [Sample, PULSE_SIZE],
}

impl Pulse {
	pub fn from_pattern ();
}
