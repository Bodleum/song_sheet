use clap::Parser;
use song_sheet::{config::Config, errors::AppError, run};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Custom config file
    #[arg(short, long, value_name = "FILE", default_value_t = String::from("config"))]
    config: String,
}

fn main() -> error_stack::Result<(), AppError> {
    // Init logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let cli = Cli::parse();

    let config = Config::read(&cli.config).unwrap();
    run(&config)
}

// #[allow(dead_code)]
// fn plain_text() -> error_stack::Result<(), AppError> {
//     let tex_path: &str = "plain_text.tex";
//     let mut latex = LaTeX::new(tex_path)?;
//
//     // Loop through directory and extract songs
//     let directory = "./Songs";
//     for entry in fs::read_dir(directory)
//         .into_report()
//         .attach_printable(format!("Could not open directory {}.", directory))
//         .change_context(DirError)
//         .change_context(AppError)?
//     {
//         let path: PathBuf = entry
//             .into_report()
//             .attach_printable(format!("Error reading files in {}.", directory))
//             .change_context(DirError)
//             .change_context(AppError)?
//             .path()
//             .as_path()
//             .to_owned();
//         if path.is_file() {
//             let mut buf = String::new();
//             File::open(&path)
//                 .into_report()
//                 .attach_printable(format!("Could not open file {:#?}.", path))
//                 .change_context(FileError)
//                 .change_context(AppError)?
//                 .read_to_string(&mut buf)
//                 .into_report()
//                 .attach_printable(format!(
//                     "Error reading {}. Not vaild UTF-8.",
//                     &path.display()
//                 ))
//                 .change_context(FileError)
//                 .change_context(AppError)?;
//             latex.add_song(parser::PlainText::parse(&buf).unwrap());
//         }
//     }
//
//     let latex = latex.write_to_file()?;
//     latex.compile("ss.tex").unwrap();
//     latex.clean().unwrap();
//
//     Ok(())
// }
