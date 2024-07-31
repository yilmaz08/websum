use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use clap::Parser;

static DEFAULT_SERVER: &str = "https://raw.githubusercontent.com/yilmaz08/websum/main/archive/%h";

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(help = "File path")]
    path: String,

    #[arg(short, long, help = "Do not print extra information")]
    short: bool,

    #[arg(short='S', long, help = "Custom Server to request, %h will be replaced with the SHA256 hash")]
    server: Option<String>,
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Box<dyn Error>> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 { break; }
        context.update(&buffer[..count]);
    }

    return Ok(context.finish());
}

fn print_error(err: std::io::Error) {
    match err.kind() {
        std::io::ErrorKind::NotFound => eprintln!("Error: File not found!"),
        std::io::ErrorKind::PermissionDenied => eprintln!("Error: Permission denied!"),
        _ => eprintln!("Unknown Error: {}", err)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse Arguments
    let args = Args::parse();

    // Print "Processing..."
    if !args.short { println!("Processing..."); }

    // SHA256
    let file = match File::open(&args.path) {
        Ok(file) => file,
        Err(err) => {
            if !args.short { print_error(err); }
            std::process::exit(1);
        }
    };
    let reader = BufReader::new(file);
    let digest = sha256_digest(reader)?;
    let sha256 = HEXUPPER.encode(digest.as_ref());
    
    // Print SHA256 Result
    if !args.short { println!("SHA256: {}", sha256); }
    
    // Request
    let server = match args.server.is_some() {
        true => args.server.unwrap(),
        false => DEFAULT_SERVER.to_string()
    };
    let url = server.replace("%h", &sha256);

    if server == url {
        if !args.short { eprintln!("URL does not contain a hash! Using WebSum like this might be misleading."); }
        std::process::exit(1);
    }

    let response = reqwest::blocking::get(url)?;

    // UNSUCCESSFUL
    if response.status() == 404 {
        if !args.short { eprintln!("This file is not found in our archive! It is either invalid or not in our archive."); }
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
