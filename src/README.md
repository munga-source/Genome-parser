# Genome_parser

A command-line tool written in Rust for summarizing genomic data in **FASTA**, **FASTQ**, and **BAM** formats.

##  Features

- Parses `.fasta`/`.fa`, `.fastq`/`.fastq.gz`, and `.bam` files
- Reports basic stats like:
  - GC content
  - Sequence length
  - Total and mapped reads
  - Average MAPQ
- Simple CLI interface


## Installation


### Option 1: Build from Source (requires Rust)

Clone the repository:

```bash
git clone https://github.com/munga-source/genome_parser.git
cd genome_parser

```
Build the release binary:

```bash
cargo build --release

```

The compiled binary will be located at:

```bash
target/release/genome_parser

```

### Option 2: Precompiled Binary

You can download precompiled binaries from the [Releases page](https://github.com/munga-source/Genome-parser/releases).

#### On Linux:
```bash
wget https://github.com/munga-source/Genome-parser/releases/download/v0.1.0/genome_parser-linux.tar.gz
tar -xzvf genome_parser-linux.tar.gz
chmod +x genome_parser-linux
./genome_parser-linux -i your_file.fastq.gz


```

 ## Usage

 Basic usage:

```bash
./genome_parser -i <input_file>

```


Example:

```bash
./genome_parser -i data/sample.fastq.gz
```