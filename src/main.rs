use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::{fs::File, io::Read};

use clap::Parser;
use error_stack::{IntoReport, Report, Result, ResultExt};
use song_sheet::{errors, latex::Latex, parser::PlainText};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let tex_path: &str = "ss.tex";
    let f: File = File::create(tex_path)
        .into_report()
        .attach_printable(format!("Could not create file {}.", tex_path))
        .change_context(errors::FileError)
        .unwrap_or_else(|e| {
            println!("{:#?}", e);
            exit(1);
        });
    let mut latex = Latex::new(f);

    // Loop through directory and extract songs
    let directory = "./Songs";
    fs::read_dir(directory)
        .into_report()
        .attach_printable(format!("Could not open directory {}.", directory))
        .change_context(errors::DirError)
        .unwrap_or_else(|e| {
            println!("{:#?}", e);
            exit(1);
        })
        .for_each(|entry| {
            let path: PathBuf = entry
                .into_report()
                .attach_printable(format!("Error reading files in {}.", directory))
                .change_context(errors::DirError)
                .unwrap_or_else(|e| {
                    println!("{:#?}", e);
                    exit(1);
                })
                .path()
                .as_path()
                .to_owned();
            if path.is_file() {
                let mut buf = String::new();
                File::open(&path)
                    .into_report()
                    .attach_printable(format!("Could not open file {:#?}.", path))
                    .change_context(errors::FileError)
                    .unwrap_or_else(|e| {
                        println!("{:#?}", e);
                        exit(1);
                    })
                    .read_to_string(&mut buf)
                    .into_report()
                    .attach_printable(format!(
                        "Error reading {}. Not vaild UTF-8.",
                        &path.display()
                    ))
                    .change_context(errors::FileError)
                    .unwrap_or_else(|e| {
                        println!("{:#?}", e);
                        exit(1);
                    });
                latex.add_song(PlainText::parse(&buf).expect("Error parsing"));
            }
        });

    let latex = latex.write_to_file().expect("Error writing to file!");
    latex.compile("ss.tex").unwrap();
    latex.clean().unwrap();

    exit(0);

    // Old
    // ==================
    // let config: Config = Default::default();
    // let mut latex: Latex = Latex::create("ss.tex").expect("Error creating latex file!");
    // latex.preamble.push_str(
    //     fs::read_to_string("Head.tex")
    //         .expect("Error reading Head.tex")
    //         .as_str(),
    // );
    // for f in config.songs {
    //     latex.body.push_str(
    //         fs::read_to_string(f)
    //             .unwrap_or_else(|err| {
    //                 // eprintln!("Error reading {}!", f);
    //                 eprintln!("{:#?}", err);
    //                 "".to_string()
    //             })
    //             .as_str(),
    //     )
    // }
    // latex.body.push_str(
    //     fs::read_to_string("Foot.tex")
    //         .expect("Error reading Foot.tex")
    //         .as_str(),
    // );
    // latex
    //     .run(
    //         Command::new("latexmk")
    //             .arg("-pdflua")
    //             .arg("-interaction=nonstopmode")
    //             .arg("ss.tex"),
    //     )
    //     .expect("Error in latexmk!");

    // latex
    //     .run(Command::new("latexmk").arg("-c"))
    //     .expect("Error cleaning!");
}
