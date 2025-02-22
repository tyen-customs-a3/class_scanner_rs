mod error;
mod models;
mod parser;
mod scanner;

use std::path::PathBuf;
use clap::Parser;
use error::Result;
use scanner::{Scanner, ScannerConfig};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to PBO file or directory to scan
    #[arg(short, long)]
    path: PathBuf,

    /// Enable debug output
    #[arg(short, long)]
    debug: bool,

    /// Enable parallel scanning
    #[arg(short, long)]
    parallel: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    let config = Scanner::builder()
        .debug(args.debug)
        .parallel(args.parallel)
        .build();
    
    let scanner = Scanner::new(config);

    if args.path.is_dir() {
        let results = scanner.scan_directory(&args.path)?;
        println!("\nScanned {} PBO files successfully", results.len());
        
        // Print summary
        let total_classes: usize = results.values()
            .map(|pbo| pbo.classes.len())
            .sum();
        println!("Found {} total classes", total_classes);
    } else {
        let result = scanner.scan_pbo(&args.path)?;
        println!("\nFound {} classes in {}", 
            result.classes.len(),
            args.path.display()
        );
    }

    Ok(())
}
