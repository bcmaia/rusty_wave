#[derive(Clone, Copy, PartialEq)]
pub enum Sample {
    NoSignal = 0,
    Low = 1,
    Mid = 2,
    High = 3,
    LowToHigh = 4,
    LowToMid = 5,
    MidToHigh = 6,
}


const MATRIX : [[Sample; 7]; 7] = [
    // last \ next |  NoSignal   		Low        			Mid        			High       			LowToHigh  			LowToMid   			MidToHigh
    /* NoSignal */ [Sample::NoSignal, 	Sample::Low, 		Sample::Mid, 		Sample::High, 		Sample::LowToHigh, 	Sample::LowToMid, 	Sample::MidToHigh],
    /* Low      */ [Sample::LowToMid,	Sample::Low, 		Sample::LowToMid, 	Sample::LowToHigh, 	Sample::LowToHigh, 	Sample::LowToMid, 	Sample::MidToHigh],
    /* Mid      */ [Sample::NoSignal, 	Sample::LowToMid,	Sample::Mid, 		Sample::MidToHigh, 	Sample::LowToHigh, 	Sample::LowToMid, 	Sample::MidToHigh],
    /* High     */ [Sample::MidToHigh, 	Sample::LowToHigh,	Sample::MidToHigh, 	Sample::High, 		Sample::LowToHigh, 	Sample::LowToMid, 	Sample::MidToHigh],
    /* LowToHigh*/ [Sample::NoSignal,	Sample::Low, 		Sample::Mid,		Sample::High, 		Sample::LowToHigh, 	Sample::LowToMid, 	Sample::MidToHigh],
    /* LowToMid */ [Sample::NoSignal, 	Sample::Low, 		Sample::Mid, 		Sample::High,		Sample::LowToHigh,	Sample::LowToMid, 	Sample::MidToHigh],
    /* MidToHigh*/ [Sample::NoSignal, 	Sample::Low, 		Sample::Mid, 		Sample::High, 		Sample::LowToHigh, 	Sample::LowToMid,	Sample::MidToHigh],
];

pub const SIGNALS : [&str; 7] = [
    "      ·      ", // 0
    "┃     ·      ", // 1
    "      ┃      ", // 2
    "      ·     ┃", // 3
    "·━━━━━·━━━━━·", // 4
    "·━━━━━·      ", // 5
    "      ·━━━━━·", // 6
];

impl Sample {
    pub fn interpolate(last: &Sample, next: &Sample) -> Sample {
        MATRIX[*last as usize][*next as usize]
    }

    pub fn render (&self, border : bool, title : &str) -> String {
		let s = SIGNALS[*self as usize];

        let result;
        if border {
            let s = s.replace(" ", "·");
            result = format!(" ··{}·· {}", s, title);
        } else {
            result = format!("   {}   {}", s, title);
        }

        result
    }
}

