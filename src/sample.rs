#[derive(Clone, Copy)]
pub enum Sample {
    NoSignal = 0,
    Low = 1,
    Mid = 2,
    High = 3,
    LowToHigh = 4,
    LowToMid = 5,
    MidToHigh = 6,
}

impl Sample {
    fn interpolate(last: &Sample, next: &Sample) -> Sample {
        let matrix = [
            // last \ next |  NoSignal   		Low        			Mid        			High       			LowToHigh  			LowToMid   			MidToHigh
            /* NoSignal */ [Sample::NoSignal, 	Sample::Low, 		Sample::Mid, 		Sample::High, 		Sample::LowToHigh, 	Sample::LowToMid, 	Sample::MidToHigh],
            /* Low      */ [Sample::LowToMid,	Sample::Low, 		Sample::LowToMid, 	Sample::LowToHigh, 	Sample::LowToHigh, 	Sample::LowToMid, 	Sample::MidToHigh],
            /* Mid      */ [Sample::NoSignal, 	Sample::LowToMid,	Sample::Mid, 		Sample::MidToHigh, 	Sample::LowToHigh, 	Sample::LowToMid, 	Sample::MidToHigh],
            /* High     */ [Sample::MidToHigh, 	Sample::LowToHigh,	Sample::MidToHigh, 	Sample::High, 		Sample::LowToHigh, 	Sample::LowToMid, 	Sample::MidToHigh],
            /* LowToHigh*/ [Sample::NoSignal,	Sample::Low, 		Sample::Mid,		Sample::High, 		Sample::LowToHigh, 	Sample::LowToMid, 	Sample::MidToHigh],
            /* LowToMid */ [Sample::NoSignal, 	Sample::Low, 		Sample::Mid, 		Sample::High,		Sample::LowToHigh,	Sample::LowToMid, 	Sample::MidToHigh],
            /* MidToHigh*/ [Sample::NoSignal, 	Sample::Low, 		Sample::Mid, 		Sample::High, 		Sample::LowToHigh, 	Sample::LowToMid,	Sample::MidToHigh],
        ];
        
        matrix[*last as usize][*next as usize]
    }

    fn print (&self, border : bool, title : &str) {
		let signals = [
			"      ┊      ", // 0
			"┇     ┊      ", // 1
			"      ┆     ┇", // 2
			"┅┅┅┅┅┅┅┅┅┅┅┅┅", // 3
			"┅┅┅┅┅┅┊      ", // 4
			"      ┊┅┅┅┅┅┅", // 5
		];
    
		let s = signals[*self as usize];
		
		if border {
			let s = s.replace(" ", "┈");
			println!("  ┈┈{}┈┈  {}", s, title);
		} else {
			println!("    {}    {}", s, title);
		}
    }
}

