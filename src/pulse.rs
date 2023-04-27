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
	pub fn from_pattern (patterns : &PulsePattern) -> Vec<Pulse> {
		let MID_INDEX = ceil(i/2);

		Pulse {
			title: patterns.title.clone();
			samples: (vec![Pulse::Low, PULSE_SIZE])
				.iter_mut().enumerate().for_each(
					|(i, x)| *x = 
						if MID_INDEX > i {
							patterns.start
						} else if MID_INDEX == i {
							Sample::Interpolate(patterns.start, patterns.end)
						} else {
							patterns.end
						}
				);
		}
	}
}
