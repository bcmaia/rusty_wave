
use term_size::dimensions_stdout;
use crossterm::{cursor, execute};
extern crate unicode_segmentation;
//use unicode_segmentation::UnicodeSegmentation;

pub fn vertical_print (lines : &Vec<String>, position : i32) {
    let mut lines = lines
        .iter()
        .map(|s| {s.clone()})
        .collect::<Vec<String>>();

    pad_strings(&mut lines);

    let (width, height) = dimensions_stdout().unwrap();

    println!("{}", (0..height).map(|_| {"\n"}).collect::<Vec<_>>().join(""));

    let mut x : usize = if 0 <= position {0} else {(-position) as usize};
    let position = position as usize;

    for w in position..(width + position) {
        let mut y : usize = 0;

        if lines.len() <= w {break;}

        let s = &lines[w];
        // let s : String = lines[w]
        //    .graphemes(true)
        //    .rev() // Reverse the order of the grapheme iterator.
        //    .collect(); // Collect all the chars into a new owned String.
        
        for c in s.chars() {
            let cursor_pos = cursor::MoveTo(x as u16, y as u16);
            execute!(std::io::stdout(), cursor_pos).unwrap();
    
            let printed_c = match c {
                '━' => '┃',
                '┃' => '━',
                _ => c,
            };

            print!("{}", printed_c);

            y += 1;
        }

        x += 1;
    }

    println!("\n");
}


fn pad_strings(strings: &mut Vec<String>) {
    // Find the maximum length of all strings in the vector
    let max_len = strings.iter().map(|s| s.len()).max().unwrap();

    // Pad each string with spaces to make them all the same length
    for s in strings {
		// Quick Fix
		//if s.contains("━·━") {s.remove(0);}
    
    	if s.len() >= max_len {continue;}
        let padding = max_len - s.len() + 2;
        if padding > 0 {
            s.push_str(&" ".repeat(padding));
            //s.push_str(&"x");
            //s.insert_str(0, "x");
        }

    }
}
