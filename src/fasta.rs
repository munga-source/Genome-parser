use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::{Context, Result};



/// Calculates the GC content of a DNA sequence

pub fn calculates_gc_content(sequence: &str) -> f64 {
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

pub fn parse_fasta<T: AsRef<Path>>(filename: T) -> Result<()> {
    let file = File::open(&filename)
        .with_context(|| format!("opening FASTA file {:?}", filename.as_ref()))?;
    
    let reader = BufReader::new(file);
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

