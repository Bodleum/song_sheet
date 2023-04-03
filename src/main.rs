use std::path::PathBuf;
use std::process::exit;
use std::{fs::File, io::Read};

use clap::Parser;
use colored::Colorize;
use song_sheet::{latex::Latex, parser::PlainText, Song};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

fn main() {
    let f: File = File::create("./ss.tex").expect("Error creating file!");
    let mut latex: Latex = Latex::new(f);

    let mut song: Song = Song::new("As the Deer Pants");
    song.set_chorus(
        "You alone are my Strength, my Shield
To You alone may my spirit yield
You alone are my heart's desire
And I long to worship Thee",
    );
    song.add_verse(
        "As the deer pants for the water
So my soul longs after Thee.
You alone are my heart's desire
And I long to worship You",
    );
    song.add_verse(
        "You're my friend and You are my brother,
Even though you are a king.
I love You more than any other,
So much more than anything.",
    );
    song.add_verse(
        "I want You more than gold or silver,
Only You can satisfy.
You alone are the real joy Giver,
And the Apple of my eye.",
    );
    song.set_order("vcvv").unwrap();
    latex.add_song(song);

    let mut because_he_lives: String = String::new();
    let fp: &str = "./Songs/BecauseHeLives.txt";
    File::open(fp)
        .unwrap()
        .read_to_string(&mut because_he_lives)
        .unwrap();
    let song: Song = PlainText::parse(because_he_lives.as_str()).unwrap_or_else(|err| {
        eprintln!(
            "{} in {}\n{}",
            "ERROR".red().bold(),
            fp.yellow().bold(),
            err
        );
        exit(1);
    });
    latex.add_song(song);

    let latex = latex.write_to_file().expect("Error writing to file!");
    latex.compile("ss.tex").unwrap();

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
    //                 eprintln!("{}", err);
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
