use std::ffi::OsStr;
use std::{fs, io};
use std::path::{Path, PathBuf};
use regex::Regex;

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

fn collect_c_files(dir: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
	for entry in fs::read_dir(dir)? {
		let entry = entry?;
		let path = entry.path();

		if path.is_dir() {
			collect_c_files(&path, out)?;
		} else if path.extension() == Some(OsStr::new("c")) {
			out.push(path);
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

// converts 'src/bla/blabla.c' in 'SRC :=		bla/blabla.c \'
fn convert_to_combined_string(files: &Vec<PathBuf>, dir_src_path: &str) -> String
{
	let mut all: String = String::new();
	let mut line = String::from("SRC :=\t\t\t");

	for f in files {
		let f = f.to_string_lossy();
		let f = f.strip_prefix(dir_src_path).unwrap_or(&f);
		let f = f.strip_prefix("/").unwrap_or(&f);
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

fn get_c_files_list(dir_src_path: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>>
{
	let mut files: Vec<PathBuf> = Vec::new();
	collect_c_files(Path::new(dir_src_path), &mut files)?;
	files.sort();
	Ok(files)
}


fn modify_makefile(makefile: &mut String, all_files: &Vec<PathBuf>, dir_src_path: &str) -> Result<(), Box<dyn std::error::Error>>
{
	let new_part = convert_to_combined_string(&all_files, dir_src_path);

	let start = find_line_start(makefile,"SRC :=").ok_or("no SRC := at line start")?;
	let end = start + find_unescaped_newline(&makefile[start..]);
	makefile.replace_range(start..end, &new_part);
	Ok(())
}

fn function_defs(all_files: &Vec<PathBuf>) -> String {
	
	let s = all_files.iter()
		.map(|f | fs::read_to_string(f)).collect::<Result<Vec<String>, _>>().unwrap().join("\n");
	let _ = fs::write("test.txt", s);
	print!("all_files: {:?}", all_files);
	panic!()
	
}

fn modify_header(header: &mut String, all_files: &Vec<PathBuf>, heder_file_name: &str) -> Result<(), Box<dyn std::error::Error>>
{
	// regex for finding in the header file but not needed
	// let def = Regex::new(r"[\w]+\t+\**\w+\([\w\s,\*\[\]]*\);\n").unwrap();
    // let start = def.find(header);
    // *header = def.replace_all(header, "").to_string();

	// start
	let start = header.find("// start").ok_or("no '// start' found")? + "// start".len() + 2;
	let end = header.find("// end").ok_or("no '// end' found")? - 2;

	let new_part = function_defs(all_files);
	header.replace_range(start..end, &new_part);

	



	

	// let include_garde_name = &heder_file_name.to_string().to_uppercase();

	// let mut found_gard_end = false;
	// for i in header.lines()
	// {
	// 	if i.contains("# define") && i.contains(include_garde_name)
	// 	{
	// 		found_gard_end = true;
	// 		continue;
	// 	}
	// 	if found_gard_end && i.trim().is_empty() {
	// 		continue;
	// 	}
	// }


	// let start = find_line_start(header,"SRC :=").ok_or("no SRC := at line start")?;
	// let end = start + find_unescaped_newline(&makefile[start..]);
	// makefile.replace_range(start..end, &new_part);



	Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
	let makefile_path: &Path = Path::new("./Makefile");

	let mut makefile: String = read_makefile(makefile_path)?;
	let dir_src_path: String = get_dir_src_path(&makefile, "DIR_SRC :=")
							.unwrap_or("src")
							.to_string();

	let all_files = &get_c_files_list(&dir_src_path)?;

	modify_makefile(&mut makefile, all_files, &dir_src_path)?;

	write_makefile(makefile_path, &makefile)?;

	let heder_file_name = "function_definitions.h";
	let heder_file_path = concat!("include/", "function_definitions.h");
	let mut header = fs::read_to_string(heder_file_path)?;

	modify_header(&mut header, all_files, heder_file_name)?;

	fs::write(heder_file_path, &header)?;

	Ok(())
}
