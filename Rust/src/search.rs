use std::fs;
use std::io;
use std::path::Path;
use regex::Regex;
use crate::format_number::format_number_kilo_by_kilo;

pub const VERSION: &str = "1.3.0";

fn search_dir(reg_ex: &Regex, parent_path: &Path) -> io::Result<(usize, usize)> {
    let mut file_count = 0;
    let mut folder_count = 0;
    
    for entry in fs::read_dir(parent_path)? {
        let entry = entry?;
        let path = entry.path();
        let obj = entry.file_name();
        
        if reg_ex.is_match(&obj.to_string_lossy()) {
            if path.is_dir() {
                folder_count += 1;
                println!("d  {:?}", path);
            } else {
                file_count += 1;
                let size = format_number_kilo_by_kilo(fs::metadata(&path)?.len() as i64);
                println!("-  {:?} (size = {} byte)", path, size);
            }
        }
        
        if path.is_dir() {
            let (folder_c, file_c) = search_dir(reg_ex, &path)?;
            folder_count += folder_c;
            file_count += file_c;
        }
    }
    
    Ok((folder_count, file_count))
}

pub fn search_under_path(regular_expression: &str, dir_path_to_search: &Path) -> io::Result<i32> {
    let reg_ex = Regex::new(regular_expression).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidInput, "Invalid regular expression")
    })?;
    
    if !dir_path_to_search.exists() || !dir_path_to_search.is_dir() {
        eprintln!("\nERROR: \n\n The process can not access the given path !\n Please check whether the path exists and whether the path is to a directory/folder.\n And check if the access to the path is permitted or not.\n");
        return Ok(-2);
    }
    
    println!("###\n\nStart path: \n {:?}", fs::canonicalize(dir_path_to_search)?);
    
    let (folder_count, file_count) = search_dir(&reg_ex, dir_path_to_search)?;
    println!("\n{} file(s),", format_number_kilo_by_kilo(file_count as i64));
    println!("{} folder(s) found", format_number_kilo_by_kilo(folder_count as i64));
    
    Ok(0)
}