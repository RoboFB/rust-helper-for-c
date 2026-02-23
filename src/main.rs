use std::ffi::OsStr;
use std::{fs, io};
use std::path::{Path};

/* 
a small to to update the makefile with all the .c files in the src directory:

*/


fn read_makefile(open: &Path) -> Result<String, std::io::Error> {
	fs::read_to_string(open)
}

fn get_dir_src_path(makefile: &str) -> Option<&str> {
	Some(makefile
		.lines()
		.find(|l| l.starts_with("DIR_SRC :="))?
		.strip_prefix("DIR_SRC :=")?
		.trim())
}

fn collect_c_files(dir: &Path, out: &mut Vec<String>) -> io::Result<()> {
	for entry in fs::read_dir(dir)? {
		let entry = entry?;
		let path = entry.path();

		if path.is_dir() {
			collect_c_files(&path, out)?;
		} else if path.extension() == Some(OsStr::new("c")) {
			out.push(path
				.to_string_lossy()
				.into_owned());
		}
	}
	Ok(())
}


fn get_stuff(open: &Path) -> Result<String, Box<dyn std::error::Error>> {
	
	let makefile: String = read_makefile(open)?;
	let dir_src_path = get_dir_src_path(&makefile)
								.ok_or("no DIR_SRC path found")?;

	let mut files = Vec::new();
	collect_c_files(Path::new(dir_src_path), &mut files)?;
	files.sort();


	// let mut nice_lock = files.join(" ");
	let mut all: String = String::new();
	let mut line = String::from("SRC :=\t\t\t");

	for f_1 in files {
		let f = f_1.strip_prefix("src/").unwrap_or(&f_1);
		if line.len() + f.len() + 1 > 80 {
			all.push_str(&line);
			all.push_str("\\\n");
			line = String::from("\t\t\t\t");
		}
		line.push_str(&f);
		line.push_str(" ");
	}
	all.push_str(&line);
	all.strip_suffix(" \\\n").unwrap_or(&all);
	all.push('\n');
	


	let at = makefile.find("\nSRC :=").expect("needs SRC :=") + 1;

	let mut new_makefile = makefile[..at].to_string();
	new_makefile.push_str(&all);
	let rest = makefile[at..].lines();
	let mut found_end = false;
	for i in rest {
		if found_end {
			new_makefile.push_str(i);
			new_makefile.push('\n');
		} else if !i.trim_end().ends_with("\\") {
			found_end = true;
		}
	}
	
	
	Ok(new_makefile)
}



fn main()
{
	// let args: Vec<String> = env::args().collect();
	

	// if args.len() > 1
	// {
	// 	println!("I HAVE ARGS NOICE!");
	// }
	// else
	// {
	// 	println!("Hello, world!");
	// }
	
	let makefile_path = Path::new("./Makefile");

	let new_makefile = get_stuff(makefile_path).unwrap();
	fs::write(makefile_path, new_makefile).unwrap();
	
}
