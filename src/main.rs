use std::ffi::OsStr;
use std::{fs, io};
use std::path::{Path};

/* 
a small to to update the makefile with all the .c files in the src directory:

*/


fn read_makefile(open: &Path) -> Result<String, std::io::Error> {
	match fs::read_to_string(open)
	{
		Ok(content) => Ok(content),
		Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(String::from("SRC :=")),
		Err(e) => Err(e)
	}
}

fn write_makefile(open: &Path, content: &str) -> Result<(), std::io::Error> {
	fs::write(open, content)
}

fn get_dir_src_path<'a>(makefile: &'a str, prefix: &str) -> Option<&'a str> {
	Some(makefile
		.lines()
		.find(|l| l.starts_with(prefix))?
		.strip_prefix(prefix)?
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


fn find_unescaped_newline(s: &str) -> usize {
	let bytes = s.as_bytes();
	for i in 0..bytes.len() -1 {
		if bytes[i] != b'\\' && bytes[i + 1] == b'\n' {
			return i + 1;
		}
	}
	bytes.len()
}

fn find_line_start(s: &str, prefix: &str) -> Option<usize> {
	s.match_indices('\n')
		.map(|(i, _)| i + 1)
		.chain(std::iter::once(0))
		.find(|&i| s[i..].starts_with(prefix))
}

fn convert_to_combined_string(files: Vec<String>, dir_src_path: &str) -> String
{
	let mut all: String = String::new();
	let mut line = String::from("SRC :=\t\t\t");

	for f in files {
		let mut f = f.strip_prefix(dir_src_path).unwrap_or(&f);
		f = f.strip_prefix("/").unwrap_or(&f);
		if line.len() + f.len() + 1 > 80 {
			all.push_str(&line);
			all.push_str("\\\n");
			line = String::from("\t\t\t\t");
		}
		line.push_str(&f);
		line.push_str(" ");
	}
	all.push_str(&line);
	all.pop();
	all
}

fn get_new_part(makefile: &str) -> Result<String, Box<dyn std::error::Error>>
{
	let dir_src_path = get_dir_src_path(&makefile, "DIR_SRC :=")
							.unwrap_or("src");

	let mut files = Vec::new();
	collect_c_files(Path::new(dir_src_path), &mut files)?;
	files.sort();
	Ok(convert_to_combined_string(files, dir_src_path))
}


fn modify_makefile(makefile: &mut String) -> Result<(), Box<dyn std::error::Error>>
{
	let all = get_new_part(makefile)?;

	let start = find_line_start(makefile,"SRC :=").ok_or("no SRC := at line start")?;
	let end = start + find_unescaped_newline(&makefile[start..]);
	makefile.replace_range(start..end, &all);
	Ok(())
}



fn main() -> Result<(), Box<dyn std::error::Error>>
{

	let makefile_path = Path::new("./Makefile");

	let mut makefile: String = read_makefile(makefile_path)?;

	modify_makefile(&mut makefile)?;

	write_makefile(makefile_path, &makefile)?;
	Ok(())
}
