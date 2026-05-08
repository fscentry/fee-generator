use std::path::{Path, PathBuf};

pub fn write_into_file(path :&str,file_name : &str, result : &Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    std::fs::create_dir_all(path)?;

    let mut full_path = PathBuf::from(path);
    full_path.push(format!("{file_name}.txt"));

    let content = result.join("\n");
    std::fs::write(&full_path, &content)?;
    println!("Wrote {} results to file", full_path.display());
    Ok(())
}