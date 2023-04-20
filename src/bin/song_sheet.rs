use std::fs;
use std::path::PathBuf;
use std::{fs::File, io::Read};

use clap::Parser;
use error_stack::{IntoReport, Result, ResultExt};
use log::{info, trace};
use song_sheet::{
    errors::{AppError, DirError, FileError},
    latex::LaTeX,
    parser,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

fn main() -> Result<(), AppError> {
    // Init logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("trace"));

    video_psalm()
}

#[allow(dead_code)]
fn video_psalm() -> Result<(), AppError> {
    let tex_path = "vs.tex";
    trace!("Creating new latex file {tex_path}.");
    let mut latex = LaTeX::new(tex_path)?;

    let song_file = "VS/BonAccordHymns.json";
    trace!("Reading {song_file} to string.");
    let buf = fs::read_to_string(song_file).unwrap();
    info!("Parsing {song_file}.");
    let songs = parser::video_psalm(&buf.trim_start_matches('\u{feff}')).unwrap();
    for song in songs {
        trace!("Adding {} to latex.", song.name);
        latex.add_song(song);
    }

    info!("Writing to LaTeX file.");
    let latex = latex.write_to_file()?;
    info!("Compiling LaTeX.");
    latex.compile(tex_path).unwrap();
    info!("Cleaning up LaTeX files.");
    latex.clean().unwrap();

    info!("Done!");
    Ok(())
}

#[allow(dead_code)]
fn plain_text() -> Result<(), AppError> {
    let tex_path: &str = "plain_text.tex";
    let mut latex = LaTeX::new(tex_path)?;

    // Loop through directory and extract songs
    let directory = "./Songs";
    for entry in fs::read_dir(directory)
        .into_report()
        .attach_printable(format!("Could not open directory {}.", directory))
        .change_context(DirError)
        .change_context(AppError)?
    {
        let path: PathBuf = entry
            .into_report()
            .attach_printable(format!("Error reading files in {}.", directory))
            .change_context(DirError)
            .change_context(AppError)?
            .path()
            .as_path()
            .to_owned();
        if path.is_file() {
            let mut buf = String::new();
            File::open(&path)
                .into_report()
                .attach_printable(format!("Could not open file {:#?}.", path))
                .change_context(FileError)
                .change_context(AppError)?
                .read_to_string(&mut buf)
                .into_report()
                .attach_printable(format!(
                    "Error reading {}. Not vaild UTF-8.",
                    &path.display()
                ))
                .change_context(FileError)
                .change_context(AppError)?;
            latex.add_song(parser::PlainText::parse(&buf).unwrap());
        }
    }

    let latex = latex.write_to_file()?;
    latex.compile("ss.tex").unwrap();
    latex.clean().unwrap();

    Ok(())
}
