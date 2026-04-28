use std::fs;
pub fn setup() {
    // make sure file is there
    println!("Setting up tests environment...");
    let _file = match fs::File::open("./config/configuration.toml"){
        Ok(file) => file,
        Err(e) => {
            println!("Error opening config file: {}", e);
            panic!();
        }
    };

}