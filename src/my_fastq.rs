use std::fs::File;
use std::path::Path;
use anyhow::{Context, Result};
use flate2::read::MultiGzDecoder;
use bio::io::fastq;
use std::io::{BufReader, Read, Seek, SeekFrom};

pub fn parse_fastq<P: AsRef<Path>>(filename: P) -> Result<()> {
    println!("Parsing FASTQ file: {:?}", filename.as_ref());

    let mut file = File::open(&filename)
        .with_context(|| format!("Failed to open FASTQ file {:?}", filename.as_ref()))?;

   
    let mut magic = [0u8; 2];
    file.read_exact(&mut magic)?;
    file.seek(SeekFrom::Start(0))?; // rewind

    let is_gzipped = magic == [0x1f, 0x8b];
    println!("Gzip magic bytes found: {is_gzipped}");

    let reader: Box<dyn Read> = if is_gzipped {
        println!("Detected gzipped FASTQ, using MultiGzDecoder...");
        Box::new(BufReader::new(MultiGzDecoder::new(file)))
    } else {
        println!("Detected plain FASTQ.");
        Box::new(BufReader::new(file))
    };

    let reader = fastq::Reader::new(reader);
    let mut found_records = false;

    for result in reader.records() {
        let record = result.with_context(|| "Error reading record from FASTQ file")?;
        found_records = true;

        let seq = record.seq();
        let qual = record.qual();

        println!("Sequence: {}", String::from_utf8_lossy(seq));
        println!("Quality:  {}", String::from_utf8_lossy(qual));
    }

    if !found_records {
        println!("No records found in the FASTQ file.");
    }

    println!("Done parsing FASTQ.");
    Ok(())
}
