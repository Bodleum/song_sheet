use std::fs;
use std::path::PathBuf;
use std::{fs::File, io::Read};

use clap::Parser;
use error_stack::{IntoReport, Result, ResultExt};
use song_sheet::{errors, latex::LaTeX, parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

fn main() -> Result<(), errors::AppError> {
    // Init logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("trace"));

    video_psalm()
}

#[allow(dead_code)]
fn video_psalm() -> Result<(), errors::AppError> {
    let tex_path = "vs.tex";
    let mut latex = LaTeX::new(tex_path)?;

    let buf = fs::read_to_string("VS/BonAccordHymns.json").unwrap();
    let songs = parser::video_psalm(&buf.trim_start_matches('\u{feff}')).unwrap();
    for song in songs {
        latex.add_song(song);
    }

    let latex = latex.write_to_file()?;
    latex.compile(tex_path).unwrap();
    latex.clean().unwrap();

    Ok(())
}

#[allow(dead_code)]
fn plain_text() -> Result<(), errors::AppError> {
    let tex_path: &str = "plain_text.tex";
    let mut latex = LaTeX::new(tex_path)?;

    // Loop through directory and extract songs
    let directory = "./Songs";
    for entry in fs::read_dir(directory)
        .into_report()
        .attach_printable(format!("Could not open directory {}.", directory))
        .change_context(errors::DirError)
        .change_context(errors::AppError)?
    {
        let path: PathBuf = entry
            .into_report()
            .attach_printable(format!("Error reading files in {}.", directory))
            .change_context(errors::DirError)
            .change_context(errors::AppError)?
            .path()
            .as_path()
            .to_owned();
        if path.is_file() {
            let mut buf = String::new();
            File::open(&path)
                .into_report()
                .attach_printable(format!("Could not open file {:#?}.", path))
                .change_context(errors::FileError)
                .change_context(errors::AppError)?
                .read_to_string(&mut buf)
                .into_report()
                .attach_printable(format!(
                    "Error reading {}. Not vaild UTF-8.",
                    &path.display()
                ))
                .change_context(errors::FileError)
                .change_context(errors::AppError)?;
            latex.add_song(parser::PlainText::parse(&buf).unwrap());
        }
    }

    let latex = latex.write_to_file()?;
    latex.compile("ss.tex").unwrap();
    latex.clean().unwrap();

    Ok(())
}
