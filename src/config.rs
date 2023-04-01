pub struct Config {
    pub chords: bool,

    pub latex_cmd: String,
    pub latex_args: Vec<String>,
    pub songs: Vec<String>,
}

impl Default for Config {
    fn default() -> Config {
        let songs = std::fs::read_dir("Songs")
            .expect("Error reading directory!")
            .map(|f| {
                f.expect("Iteration error!")
                    .path()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();

        Config {
            chords: true,

            latex_cmd: "latexmk".to_string(),
            latex_args: vec![
                "-pdflua".to_string(),
                "-interaction=nonstopmode".to_string(),
            ],
            songs,
        }
    }
}
