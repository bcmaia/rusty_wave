mod bit;
mod sample;
mod pulse;

use bit::Bit;
use sample::Sample;
use pulse::{PulsePattern, Pulse};


type LineEncoding = fn(Bit, &mut Bit) -> Sample;
type BlockEncoding = fn([])

fn nrz (
	b : Bit,
	state : &mut Bit
) -> Sample {
	match b {
		Bit::Zero => PulsePatern(
			title: String::from("0"),
			start: Sample::Low,
			end: Sample::Low,
		),
		Bit::One => PulsePatern
	}
}
