use std::process::Command;
use std::str;
use std::env::args;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let path = args().nth(1).expect("No file path provided");

    println!("Processing...");
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("Failed to execute sha256sum");

    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).expect("Failed to read stderr");
        eprintln!("Error: {}", stderr);
        return Ok(());
    }

    let stdout = str::from_utf8(&output.stdout).expect("Failed to read stdout");
    let sha256 = stdout.split_whitespace().next().expect("Failed to get sha256sum").to_string().to_uppercase();
    println!("SHA256SUM: {}", sha256);

    let url = format!("https://raw.githubusercontent.com/yilmaz08/websum/main/archive/{}", sha256);
    
    let response = reqwest::blocking::get(url)?;

    if response.status() == 404 {
        eprintln!("This file is not found in our archive! It is either invalid or not in our archive.");
        return Ok(());
    }

    println!("This file is a valid \"{}\" file.", response.text()?);

    return Ok(());
}
