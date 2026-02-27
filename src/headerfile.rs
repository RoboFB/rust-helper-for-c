use regex::Regex;
use std::fs;
use std::path::PathBuf;

fn function_defs(all_files: &Vec<PathBuf>) -> Result<String, std::io::Error> {
    let magic = Regex::new(r"(static\s*|const\s*)*[\w]+([\*\s])+\w+\([^\)]*\)\s*\{").unwrap();

    let c_files_conntet = all_files
        .iter()
        .map(|f| fs::read_to_string(f))
        .collect::<Result<Vec<String>, _>>()?
        // .unwrap()
        .join("\n");

    let found_regex_lines = magic.find_iter(&c_files_conntet);
    let mut all_matches = String::new();
    for i in found_regex_lines {
        let i = i.as_str();
        if i.contains("static") || i.contains("main") {
            continue;
        }
        all_matches.push_str(i);
        all_matches = all_matches.replace("\n{", ";\n");
    }
    Ok(all_matches)
}

pub fn modify_header(
    header: &mut String,
    all_files: &Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    // let def = Regex::new(r"[\w]+\t+\**\w+\([\w\s,\*\[\]]*\);\n").unwrap();
    let start = header.find("// start\n").ok_or("no '// start\\n' found")? + "// start\n".len();
    let end = header.find("// end\n").ok_or("no '// end\\n' found")?;

    let new_part = function_defs(all_files)?;
    header.replace_range(start..end, &new_part);
    Ok(())
}
