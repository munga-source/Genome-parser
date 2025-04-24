use genome_parser::fasta::calculates_gc_content;



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
