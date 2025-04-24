use bio::io::fastq;
use std::fs::File;
use std::path::Path;
use anyhow::{Context, Result};
use flate2::read::MultiGzDecoder;
use std::io::{BufReader, Read};

pub fn parse_fastq<P: AsRef<Path>>(filename: P) -> Result<()> {
    println!("Parsing FASTQ file: {:?}", filename.as_ref());

    let file = File::open(&filename)
        .with_context(|| format!("Failed to open FASTQ file {:?}", filename.as_ref()))?;

    let reader: Box<dyn Read> = if filename.as_ref().extension() == Some("gz".as_ref()) {
        println!("Detected a gzipped file, using MultiGzDecoder...");
        let gz = MultiGzDecoder::new(file);
        Box::new(BufReader::new(gz))
    } else {
        Box::new(BufReader::new(file))
    };

    let reader = fastq::Reader::new(reader);
    let mut found_records = false;

    for record in reader.records() {
        let record = record.with_context(|| "Error reading record from FASTQ file")?;
        found_records = true;

        let seq: &[u8] = record.seq();
        let qual: &[u8] = record.qual();

        let seq_str = String::from_utf8_lossy(seq);
        let qual_str = String::from_utf8_lossy(qual);

        println!("Sequence:  {}", seq_str);
        println!("Quality:   {}", qual_str);
    }

    if !found_records {
        println!("No records found in the FASTQ file.");
    }

    println!("Done parsing FASTQ.");
    Ok(())
}
