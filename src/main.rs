use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Calculates the GC content of a DNA sequence

fn calculates_gc_content(sequence: &str) -> f64 {
    let gc_count = sequence
        .chars()
        .filter(|&base| base == 'G' || base == 'C')
        .count();
    let x = (gc_count as f64 / sequence.len() as f64) * 100.0;
    if x.is_nan(){
        panic!("Seems there's no string");
    }
    return x;
}

/// Parses a FASTA file and calculates statistics for each sequence.

fn parse_fasta<T: AsRef<Path>>(filename: T) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut header = String::new();
    let mut sequence = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('>') {
            if !header.is_empty() {
                // Print stats for the previous sequence
                let gc_content = calculates_gc_content(&sequence);
                println!("Header: {}", header);
                println!("Sequence Length: {}", sequence.len());
                println!("GC Content: {:.2}%", gc_content);
                sequence.clear();
            }
            header = line;
        } else {
            sequence.push_str(&line);
        }
    }

    // Print stats for the last sequence

    if !header.is_empty() {
        let gc_content = calculates_gc_content(&sequence);
        println!("Header: {}", header);
        println!("Sequence Length: {}", sequence.len());
        println!("GC Content: {:.2}%", gc_content);
    }

    Ok(())
}

use clap::Arg;

fn main() -> std::io::Result<()> {
    let cli = clap::Command::new("genome_parser");
    let input_arg = Arg::new("input");
    let input_arg = input_arg.help("Input filename in fasta format.").short('i').long("input").required(true);
    let cli = cli.arg(input_arg).arg_required_else_help(true);

    let matches = cli.get_matches();
    let fasta_file = matches.get_one::<String>("input").unwrap();

    parse_fasta(fasta_file)?; // This should call the parse_fasta function
    

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gccontent() {
        let x = calculates_gc_content("ATCG");
        assert_eq!(x, 50.0);
    }
    #[test]
    #[should_panic]
    fn test_empty_string(){
        let _x = calculates_gc_content("");
    
    }
}
