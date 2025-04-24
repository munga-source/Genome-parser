# genome_parser

A command-line tool written in Rust for summarizing genomic data in **FASTA**, **FASTQ**, and **BAM** formats.

##  Features

- Parses `.fasta`, `.fastq`/`.fastq.gz`, and `.bam` files
- Reports basic stats like:
  - GC content
  - Sequence length
  - Total and mapped reads
  - Average MAPQ
- Simple CLI interface

## 🚀 Installation

### Option 1: Build from Source (requires Rust)

```bash
git clone https://github.com/munga-source/genome_parser.git
cd genome_parser
cargo build --release

The binary will be located at target/release/genome_parser.




### Option 2: Precompiled Binary

Precompiled binaries for Linux, macOS, and Windows will be available in the Releases section.

Download the binary and run:

chmod +x genome_parser
./genome_parser -i your_file.fastq.gz


### Usage

./genome_parser -i <input_file>


Supported input file types:

    .bam

    .fastq, .fastq.gz

    .fasta, .fa

Example:

./genome_parser -i data/sample.fastq.gz
