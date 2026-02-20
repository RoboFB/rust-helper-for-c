use std::io;
use rand::Rng;

use std::env;
use std::fs;

/* 
a small to to update the makefile with all the .c files in the src directory:



*/



fn main() -> std::io::Result<()>
{
	let args: Vec<String> = env::args().collect();
	

	if args.len() > 1
	{
		println!("I HAVE ARGS NOICE!");
	}
	else
	{
		println!("Hello, world!");
	}

	let makefile_opt = fs::read_to_string("./Makefile")?;
	
	// println!("Makefile content:\n{}", makefile_opt);

	let ma = makefile_opt.matches("SRC :=");

	for line 
	println!("Matches for SRC: {}", ma.);

	Ok(())
}
	


#[allow(dead_code)]
fn random_test() -> i32
{
	rand::thread_rng().gen_range(1..=100)
}

#[allow(dead_code)]
fn old_testings()
{

	println!("Hello, world!");

	let mut guess = String::new();

	let len: usize = io::stdin()
	.read_line(&mut guess)
	.expect("Failed to read line");

	// guess = guess[0..len-1];
	let without_newline: &str = &guess[0..len-1];
	println!("You guessed: {}, length: {}", without_newline, len);


	let what = random_test();

	println!("bla bla {what}");
}