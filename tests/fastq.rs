use genome_parser::my_fastq::parse_fastq;
use std::path::Path;



#[test]

fn test_parse_fastq() {
    let fastq_file = Path::new("tests/data/test.fastq.gz");
    let result = parse_fastq(fastq_file);

    assert!(result.is_ok(), "parse_fastq should not return an error" );




}

