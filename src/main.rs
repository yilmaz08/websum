use std::{
    env::args,
    error::Error,
    fs::File,
    io::{BufReader, Read},
};
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Box<dyn Error>> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    return Ok(context.finish());
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = args().nth(1).expect("No file path provided");

    println!("Processing...");

    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let digest = sha256_digest(reader)?;

    let sha256 = HEXUPPER.encode(digest.as_ref());
    
    println!("SHA256: {}", sha256);

    let url = format!("https://raw.githubusercontent.com/yilmaz08/websum/main/archive/{}", sha256);
    
    let response = reqwest::blocking::get(url)?;

    if response.status() == 404 {
        eprintln!("This file is not found in our archive! It is either invalid or not in our archive.");
        return Ok(());
    }

    println!("This file is a valid \"{}\" file.", response.text()?);

    return Ok(());
}
