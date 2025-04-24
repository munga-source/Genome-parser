use std::path::Path;
use anyhow::{Context, Result};
use rust_htslib::bam::{Reader, Read};
use std::str;




pub fn parse_bam<P: AsRef<Path>>(filename: P) -> Result<()> {
    let mut bam = Reader::from_path(&filename)
        .with_context(|| format!("opening BAM file {:?}", filename.as_ref()))?;


    let header = bam.header().to_owned();
    let mut total = 0u64;
    let mut mapped = 0u64;
    let mut sum_mapq = 0u64;

    for record in bam.records() {
        let rec = record?;
        total += 1;
        if !rec.is_unmapped() {
            mapped += 1;
            sum_mapq += rec.mapq() as u64;    
        }
    }
    let pct_mapped = if total > 0  {
        100.0 * (mapped as f64 / total as f64)
    } else {
        0.0
    };    

    let avg_mapq = if mapped > 0 {
        sum_mapq as f64 / mapped as f64
    } else {
        0.0
    };

    let pct_unmapped = 100.0 - pct_mapped; 
    let unmapped = total - mapped;  

    println!("BAM targets   :");
    for name in header.target_names() {
        match str::from_utf8(name) {
            Ok(s) => println!(" -{}", s),
            Err(_) => println!(" - [Invalid UTF-8]"),
        }
    }
    println!("Total reads   : {}", total);
    println!("Mapped reads  : {} ({:.2}%)", mapped, pct_mapped);
    println!("Average MAPQ  : {:.2}", avg_mapq);
    println!("Unmapped reads  : {} ({:.2})", unmapped, pct_unmapped);
    Ok(())
}  








