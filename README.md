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

1. Clone the repository:

    ```bash
    git clone https://github.com/munga-source/genome_parser.git
    cd genome_parser
    ```
2. Build the release binary:

    ```bash
    cargo build --release
    ```

3. The compiled binary will be located at:

    ```bash
    target/release/genome_parser
    ```

### Option 2: Precompiled Binary for macOS

You can download precompiled binaries from the [Releases page](https://github.com/munga-source/Genome-parser/releases).

- **macOS**: `genome_parser-macos.tar.gz`

#### Example: Downloading the macOS Binary

1. Download the binary:
    ```bash
    wget https://github.com/munga-source/Genome-parser/releases/download/v0.1.0/genome_parser-macos.tar.gz
    ```

2. Extract the archive:
    ```bash
    tar -xzvf genome_parser-macos.tar.gz
    ```

3. Make it executable:
    ```bash
    chmod +x genome_parser-macos
    ```

4. Run the tool:
    ```bash
    ./genome_parser-macos --help
    ```



 ## Usage

1. Basic usage:

    ```bash
    ./genome_parser -i <input_file>
    ```


2. Example:

    ```bash
    ./genome_parser -i data/sample.fastq.gz
    ```
