use genome_parser::fasta;
use genome_parser::bam;
use genome_parser::my_fastq;


use anyhow::Result;
use clap::{Arg, Command};

fn main() -> Result<()> {
    let matches = Command::new("Genome_parser")
        .about("Computes stats on FASTA,BAM and FASTQ")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Input filename (FASTA OR BAM OR FASTQ)")
                .required(true)
        )
        .get_matches();

    let filename = matches.get_one::<String>("input").unwrap();


    if filename.ends_with(".bam") {
        bam::parse_bam(filename)?;

    } else if filename.ends_with(".fastq") || filename.ends_with(".fastq.gz") {
        println!("Parsing FASTQ file: {}", filename);
        my_fastq::parse_fastq(filename)?;
    
    } else if filename.ends_with(".fasta") || filename.ends_with(".fa") {
        fasta::parse_fasta(filename)?;
        
    } else {
        println!("Unsupported file type.");

    }

    Ok(())
}



