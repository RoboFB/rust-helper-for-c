use std::io;

fn main() {
    println!("Hello, world!");

	let mut guess = String::new();

	let len: usize = io::stdin()
	.read_line(&mut guess)
	.expect("Failed to read line");

	// guess = guess[0..len-1];
	let without_newline: &str = &guess[0..len-1];
	println!("You guessed: {}, length: {}", without_newline, len);

	// let mut x: i32 = 5;
	// let & y = x;

	// println!("bob:{x},{y}");
	
	// x=10;
	// // y=99;

	// println!("bob:{x},{y}");
}
