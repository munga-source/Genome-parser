


use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Function to parse a FASTA file and collect sequences

fn parse_fasta<T: AsRef<Path>>(filename: T) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut header = String::new();
    let mut sequence = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('>') {
            // Print the previous sequence before starting a new one
            if !header.is_empty() {
                println!("Header: {}", header);
                println!("sequence: {}", sequence);
                sequence.clear();
            }
            header = line;
        } else {
            sequence.push_str(&line);
        }
    }

    // Print the last sequence

    if !header.is_empty() {
        println!("Header: {}", header);
        println!("Sequence: {}", sequence);
    }

    Ok(())
}



fn main() -> std::io::Result<()> {
    let fasta_file = std::env::args().nth(1);
    match fasta_file{
        None => println!("Please give file name"),
        Some(file) => parse_fasta(file)?, // This should call the parse_fasta function
    }
    
    Ok(())
}


