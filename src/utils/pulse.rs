// mod sample;

use super::sample::Sample;

pub const PULSE_SIZE : usize = 6usize; 
pub const MID_INDEX : usize = ((PULSE_SIZE as f32) / 2f32 + 0.5f32) as usize;

pub struct PulsePattern {
	pub title : String,
	pub start : Sample,
	pub end : Sample,
}

pub struct Pulse {
	pub title: String,
	pub samples : [Sample; PULSE_SIZE],
}

impl Pulse {
	pub fn from_pattern (patterns : &PulsePattern) -> Pulse {
		let mid_index = ((PULSE_SIZE as f32) / 2f32).ceil() as usize;

		let mut samples = [Sample::Low; PULSE_SIZE];
		samples
			.iter_mut()
			.enumerate()
			.for_each(
				|(i, x)| {*x = 
					if mid_index > i {
						patterns.start
					} else if mid_index == i {
						Sample::interpolate(&patterns.start, &patterns.end)
					} else {
						patterns.end
					}
				}
		);

		Pulse {
			title: patterns.title.clone(),
			samples: samples,
		}
	}

	pub fn separate_samples (&self) -> Vec<Sample> {
		self.samples.to_vec()
	}
}
