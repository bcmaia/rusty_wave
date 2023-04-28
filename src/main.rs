use term_size::dimensions_stdout;

fn main() {
    let (width, height) = dimensions_stdout().unwrap();
    println!("Terminal size: {} x {}", width, height);
}
