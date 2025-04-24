use genome_parser::bam::parse_bam;
use std::path::Path;


#[test]
fn test_parse_bam() {
    let bam_file = Path::new("tests/data/example.bam");
    let result = parse_bam(bam_file);

    assert!(result.is_ok(), "parse_bam should not return an error");

}
