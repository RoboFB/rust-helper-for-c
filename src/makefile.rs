use std::path::PathBuf;

fn find_line_start(s: &str, prefix: &str) -> Option<usize> {
    s.match_indices('\n')
        .map(|(i, _)| i + 1)
        .chain(std::iter::once(0))
        .find(|&i| s[i..].starts_with(prefix))
}

fn find_unescaped_newline(s: &str) -> usize {
    let bytes = s.as_bytes();
    for i in 0..bytes.len() - 1 {
        if bytes[i] != b'\\' && bytes[i + 1] == b'\n' {
            return i + 1;
        }
    }
    bytes.len()
}

// converts 'src/bla/blabla.c' in 'SRC :=		bla/blabla.c \'
fn convert_to_combined_string(files: &Vec<PathBuf>, dir_src_path: &str) -> String {
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

pub fn modify_makefile(
    makefile: &mut String,
    all_files: &Vec<PathBuf>,
    dir_src_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let new_part = convert_to_combined_string(&all_files, dir_src_path);

    let start = find_line_start(makefile, "SRC :=").ok_or("no SRC := at line start")?;
    let end = start + find_unescaped_newline(&makefile[start..]);
    makefile.replace_range(start..end, &new_part);
    Ok(())
}
