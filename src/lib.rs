use std::fs::File;
use std::io::{self, Write};
use std::process;

struct Config {}

impl Config {}

pub fn copy_file<W>(source: &str, dest: &mut W)
where
    W: Write + ?Sized,
{
    io::copy(
        &mut File::open(source).unwrap_or_else(|err| {
            eprintln!("Error opening {}!", source);
            eprintln!("{}", err);
            process::exit(1);
        }),
        dest,
    )
    .unwrap_or_else(|err| {
        eprintln!("Error copying {}!", source);
        eprintln!("{}", err);
        process::exit(1);
    });
}
