use std::fs;
use std::path::{Path, PathBuf};

pub fn write_into_file(path :&str,file_name : &str, result : &[String]) -> Result<(), Box<dyn std::error::Error>>{
    fs::create_dir_all(path)?;

    let mut full_path = PathBuf::from(path);
    full_path.push(file_name);

    let content = result.join("\n");
    std::fs::write(&full_path, &content)?;
    println!("Wrote {} results to file", full_path.display());
    Ok(())
}


pub fn move_file(origin: &str, destination: &str, is_backup : bool) -> Result<(), Box<dyn std::error::Error>> {
    let origin_path = Path::new(origin);
    let destination_path = Path::new(destination);

    fs::create_dir_all(destination_path)?;

    let final_path: PathBuf = if is_backup {
        let stem = origin_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid file name")?;
        let ext = origin_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let file_name = if ext.is_empty() {
            format!("backup_{}", stem)
        } else {
            format!("backup_{}.{}", stem, ext)
        };
        destination_path.join(file_name)
    } else {
        let original_name = origin_path
            .file_name()
            .ok_or("Missing file name")?;
        destination_path.join(original_name)
    };
    if is_backup {
        fs::rename(origin_path, &final_path)?;
        println!("File moved to: {}", final_path.display());
    }

    Ok(())
}