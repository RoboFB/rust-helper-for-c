use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

mod headerfile;
mod makefile;

fn read_makefile(open: &Path) -> Result<String, std::io::Error> {
    match fs::read_to_string(open) {
        Ok(content) => Ok(content),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(String::from("SRC :=")),
        Err(e) => Err(e),
    }
}

fn get_dir_src_path<'a>(makefile: &'a str, prefix: &str) -> Option<&'a str> {
    Some(
        makefile
            .lines()
            .find(|l| l.starts_with(prefix))?
            .strip_prefix(prefix)?
            .trim(),
    )
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

fn get_c_files_list(dir_src_path: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files: Vec<PathBuf> = Vec::new();
    collect_c_files(Path::new(dir_src_path), &mut files)?;
    files.sort();
    Ok(files)
}

fn are_files_equal(path1: &str, path2: &str) -> std::io::Result<bool> {
    let content1 = fs::read(path1)?;
    let content2 = fs::read(path2)?;
    
    Ok(content1 == content2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let makefile_path: &Path = Path::new("./Makefile");

    let mut makefile_str: String = read_makefile(makefile_path)?;
    let dir_src_path: String = get_dir_src_path(&makefile_str, "DIR_SRC :=")
        .unwrap_or("src")
        .to_string();

    let all_files = &get_c_files_list(&dir_src_path)?;

    makefile::modify_makefile(&mut makefile_str, all_files, &dir_src_path)?;

    fs::write(makefile_path, &makefile_str)?;

    let heder_file_path = "include/function_definitions.h";
    let heder_file_path_tmp = "include/function_definitions_tmp.h";
    let mut header = fs::read_to_string(heder_file_path)?;

    headerfile::modify_header(&mut header, all_files)?;

    fs::write(heder_file_path_tmp, &header)?;
	
	Command::new("c_formatter_42")
		.arg(heder_file_path_tmp)
		.output()?;

	if are_files_equal(heder_file_path, heder_file_path_tmp)? {
		fs::remove_file(heder_file_path_tmp)?;
	} else {
		fs::rename(heder_file_path_tmp, heder_file_path)?;
	}





    Ok(())
}
