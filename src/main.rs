use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::{self, Command};

use clap::Parser;
use song_sheet::copy_file;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let config = match cli.config {
        Some(config) => fs::read_to_string(config).unwrap_or_else(|err| {
            eprintln!("Error reading config file!");
            eprintln!("{}", err);
            process::exit(1);
        }),
        None => fs::read_to_string("./config").unwrap_or_else(|err| {
            eprintln!("Please provide a configuration file!");
            eprintln!("{}", err);
            process::exit(1);
        }),
    };

    // Create tex file
    let tex_file = File::create("ss.tex").unwrap_or_else(|err| {
        eprintln!("Failed to create TeX file!");
        eprintln!("{}", err);
        process::exit(1);
    });
    let mut stream = BufWriter::new(tex_file);

    copy_file("./Head.tex", &mut stream);

    if let Ok(entries) = fs::read_dir("Songs") {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    match entry.path().to_str() {
                        Some(s) => copy_file(s, &mut stream),
                        None => eprintln!("Invalid filname {}", entry.path().display()),
                    }
                }
            }
        }
    }

    copy_file("./Foot.tex", &mut stream);

    // Flush stream
    stream.flush().expect("Error flushing stream!");

    // Compile
    Command::new("latexmk")
        .arg("-pdflua")
        .arg("-interaction=nonstopmode")
        .arg("ss.tex")
        .output()
        .expect("Error in latexmk");

    // Clean
    Command::new("latexmk")
        .arg("-c")
        .output()
        .expect("Error in cleaning TeX files!");
}
