use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(help = "File path")]
    path: String,

    #[arg(short, long, help = "Do not print extra information")]
    short: bool,
}

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
    // Parse Arguments
    let args = Args::parse();
    
    // Check if the file exists
    if !std::path::Path::new(&args.path).exists() {
        eprintln!("Error: The file was not found!");
        std::process::exit(1);
    }

    // Print "Processing..."
    if !args.short {
        println!("Processing...");
    }

    // SHA256
    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);
    let digest = sha256_digest(reader)?;
    let sha256 = HEXUPPER.encode(digest.as_ref());
    
    // Print SHA256 Result
    if !args.short {
        println!("SHA256: {}", sha256);
    }
    
    // Request
    let url = format!("https://raw.githubusercontent.com/yilmaz08/websum/main/archive/{}", sha256);
    let response = reqwest::blocking::get(url)?;

    // UNSUCCESSFUL
    if response.status() == 404 {
        if !args.short {
            eprintln!("This file is not found in our archive! It is either invalid or not in our archive.");
        }
        std::process::exit(1);
    }

    // SUCCESSFUL
    if args.short {
        println!("{}", response.text()?);
        return Ok(());
    }
    
    println!("This file is a valid \"{}\" file.", response.text()?);
    return Ok(());
}
