
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde_json::Value;

pub fn load_file_input_txt(path : &str) -> Result<Vec<Value>, Box<dyn std::error::Error>> {

    let file = File::open(path).map_err(|e| format!("Failed opening body file '{}' : {}", path, e))?;
    let reader = BufReader::new(file);

    let mut bodies = Vec::new();
    let mut errors = Vec::new();
    for (index, line_result) in reader.lines().enumerate() {
        let line_number = index + 1;
        let line = line_result.map_err(|e| format!("Error reading line '{}' : {}", line_number, e))?;

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        match serde_json::from_str::<Value>(trimmed_line) {
            Ok(value) => bodies.push(value),
            Err(e) => {
                let snippet = if trimmed_line.len() > 60 { &trimmed_line[0..60] } else { trimmed_line };
                errors.push(format!("line {} {} → \"{}\"",line_number,e, snippet));

            }
        }
    }
    if !errors.is_empty() {
        return Err(
            format!(
                "Found '{}' invalid Json line in '{}' : \n{}",
                errors.len(),
                path,
                errors.join("\n")
            ).into());
    }
    if bodies.is_empty(){
        return Err(format!("File '{}' does not have any valid JSON lines", path).into());
    }
    println!("📄 Body file '{}' loaded: {} request bodies", path, bodies.len());
    Ok(bodies)
}