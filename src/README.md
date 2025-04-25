# genome_parser

A command-line tool written in Rust for summarizing genomic data in **FASTA**, **FASTQ**, and **BAM** formats.

## ✨ Features

- Parses `.fasta`, `.fastq`/`.fastq.gz`, and `.bam` files
- Reports basic stats like:
  - GC content
  - Sequence length
  - Total and mapped reads
  - Average MAPQ
- Simple CLI interface

<<<<<<< HEAD
## Installation
=======
---

## 🚀 Installation
>>>>>>> 93a6b65 (Update README with usage and precompile instructions)

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

Option 2: Use Precompiled Binary

Precompiled binaries for Linux, macOS, and Windows will be available in the Releases section.

After downloading the binary:

```bash 
chmod +x genome_parser
./genome_parser -i your_file.fastq.gz

```

🧪 Usage

Basic usage:

```bash
./genome_parser -i <input_file>

```

Supported Input File Types:

    .bam

    .fastq, .fastq.gz

    .fasta, .fa

Example:

```bash
./genome_parser -i data/sample.fastq.gz
```


