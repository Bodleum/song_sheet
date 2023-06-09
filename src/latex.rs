use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
    process::{Command, Output},
};

use crate::{config::Config, error::LaTeXError, Song};

/// Represents a LaTeX package
#[derive(Debug, Default)]
pub struct Package {
    pub name: String,
    pub opts: Option<Vec<String>>,
}

impl TryFrom<&[&str]> for Package {
    type Error = LaTeXError;

    fn try_from(args: &[&str]) -> Result<Self, Self::Error> {
        let name = args
            .iter()
            .next()
            .ok_or(LaTeXError::PackageError(format!(
                "Not enough arguments for package. Expected at least 1, got {}.",
                args.len()
            )))?
            .to_string();

        let opts: Vec<String> = args.iter().skip(1).map(|opt| opt.to_string()).collect();

        Ok(Package {
            name,
            opts: (!opts.is_empty()).then_some(opts),
        })
    }
}

/// Represents a LaTeX file
pub struct LaTeX<'a> {
    path: PathBuf,
    config: &'a Config,
}

pub struct LaTeXBuilder<'a> {
    file: File,
    path: PathBuf,
    doc_class: String,
    doc_opts: Vec<String>,
    version: (u8, u8, u8),
    packages: Vec<Package>,
    verse_fmt: String,
    chorus_fmt: String,
    bridge_fmt: String,
    cover: String,
    preamble_extra: Option<String>,
    songs: Vec<Song>,

    config: &'a Config,
}

impl<'a> LaTeX<'a> {
    pub fn builder_default(config: &'a Config) -> Result<LaTeXBuilder<'a>, LaTeXError> {
        let path = PathBuf::from(format!("{}.tex", &config.name));
        let file = File::create(&path).map_err(|source| LaTeXError::CreateFileError {
            path: path.display().to_string(),
            source,
        })?;

        let builder = LaTeXBuilder {
            file,
            path,
            doc_class: String::from("article"),
            doc_opts: vec![
                String::from("a4paper"),
                String::from("twoside"),
                String::from("titlepage"),
            ],
            version: (1, 0, 0),
            packages: Vec::new(),
            verse_fmt: String::new(),
            chorus_fmt: String::from(r"\quad\textit"),
            bridge_fmt: String::from(r"\textit"),
            cover: format!("\\includepdf{{./{}}}", &config.cover_image),
            preamble_extra: None,
            songs: Vec::<Song>::new(),

            config,
        };

        // Add default packages
        builder
            .use_package_str(&["geometry", "left=1cm", "right=1cm", "top=1cm", "bottom=2cm"])?
            .use_package_str(&["hyperref", "hyperindex"])?
            .use_package_str(&["makeidx"])?
            .use_package_str(&["pdfpages"])?
            .use_package_str(&["fancyhdr"])?
            .use_package_str(&["graphicx"])?
            .use_package_str(&["adjustbox"])?
            .use_package_str(&["multicol"])?
            .use_package_str(&["totcount"])?
            .use_package_str(&["xcolor"])
    }

    pub fn execute(&self, cmd: &mut Command) -> Result<Output, LaTeXError> {
        cmd.output().map_err(LaTeXError::IOError)
    }

    pub fn compile(&self) -> Result<Output, LaTeXError> {
        Command::new(&self.config.latex_cmd)
            .args(&self.config.latex_args)
            .arg(&self.path)
            .output()
            .map_err(LaTeXError::IOError)
    }

    pub fn clean(&self) -> Result<Output, LaTeXError> {
        if !self.config.keep_tex_file {
            fs::remove_file(&self.path)?;
        }

        Command::new(&self.config.latex_cmd)
            .arg("-c")
            .output()
            .map_err(LaTeXError::IOError)
    }
}

impl<'a> LaTeXBuilder<'a> {
    pub fn write_to_file(self) -> Result<LaTeX<'a>, LaTeXError> {
        self.internal_write_to_file()
    }

    /// Wrapped by `write_to_file`.
    fn internal_write_to_file(self) -> Result<LaTeX<'a>, LaTeXError> {
        {
            // Create buffered writer
            let mut stream = BufWriter::new(&self.file);

            // Write document class
            writeln!(
                stream,
                r"\documentclass[{}]{{{}}}",
                self.doc_opts.join(", "),
                self.doc_class
            )?;

            // Version
            writeln!(stream)?;
            writeln!(
                stream,
                r"\def\ssver{{{}.{}.{}}}",
                self.version.0, self.version.1, self.version.2
            )?;

            // Write packages
            writeln!(stream)?;
            writeln!(stream, "% ====   Packages   ====")?;
            for p in &self.packages {
                write!(stream, r"\usepackage")?;
                if let Some(opts) = &p.opts {
                    write!(stream, "[{}]", opts.join(", "))?;
                }
                writeln!(stream, "{{{}}}", p.name)?;
            }

            // Index
            writeln!(stream)?;
            writeln!(stream, "% ====   Index   ====")?;
            writeln!(stream, r"\makeindex")?;

            // Counters
            writeln!(stream)?;
            writeln!(stream, "% ====   Counters   ====")?;
            writeln!(stream, r"\newtotcounter{{songcount}}")?;
            writeln!(stream, r"\newtotcounter{{psalmcount}}")?;
            writeln!(stream, r"\definecolor{{title dark}}{{HTML}}{{7E73A7}}")?;

            // Footer
            writeln!(stream)?;
            writeln!(stream, "% ====   Footer   ====")?;
            writeln!(stream, r"\pagestyle{{fancy}}")?;
            writeln!(stream, r"\fancyhf{{}}")?;
            writeln!(stream, r"\cfoot{{{{\small\thepage}} \\ v{{\ssver}}}}")?;
            writeln!(stream, r"\renewcommand{{\headrulewidth}}{{0pt}}")?;

            // Expert LaTeX mode
            {
                writeln!(stream)?;
                writeln!(stream, r"\makeatletter")?;

                // Verse
                writeln!(stream)?;
                writeln!(stream, "% ====   Verse   ====")?;
                writeln!(stream, r"\renewcommand{{\verse}}{{\@versei}}")?;
                writeln!(
                    stream,
                    r"\newcommand{{\@versei}}{{\@ifnextchar\end{{\@verseend}}{{\@verseii}}}} % chktex 10"
                )?;
                writeln!(
                    stream,
                    r"\newcommand{{\@verseii}}[1]{{{}#1\par\@versei}}",
                    self.verse_fmt
                )?;
                writeln!(stream, r"\newcommand{{\@verseend}}[1]{{\vskip1em}}")?;

                // Chorus
                writeln!(stream)?;
                writeln!(stream, "% ====   Chorus   ====")?;
                writeln!(stream, r"\newcommand{{\chorus}}{{\@chorusi}}")?;
                writeln!(
                    stream,
                    r"\newcommand{{\@chorusi}}{{\@ifnextchar\end{{\@chorusend}}{{\@chorusii}}}} % chktex 10"
                )?;
                writeln!(
                    stream,
                    r"\newcommand{{\@chorusii}}[1]{{{}{{#1}}\par\@chorusi}}",
                    self.chorus_fmt
                )?;
                writeln!(stream, r"\newcommand{{\@chorusend}}[1]{{\vskip1em}}")?;

                // Bridge
                writeln!(stream)?;
                writeln!(stream, "% ====   Bridge   ====")?;
                writeln!(stream, r"\newcommand{{\bridge}}{{\@bridgei}}")?;
                writeln!(
                    stream,
                    r"\newcommand{{\@bridgei}}{{\@ifnextchar\end{{\@bridgeend}}{{\@bridgeii}}}} % chktex 10"
                )?;
                writeln!(
                    stream,
                    r"\newcommand{{\@bridgeii}}[1]{{{}{{#1}}\par\@bridgei}}",
                    self.bridge_fmt
                )?;
                writeln!(stream, r"\newcommand{{\@bridgeend}}[1]{{\vskip1em}}")?;

                writeln!(stream)?;
                writeln!(stream, r"\makeatother")?;
            }

            // Song environment
            writeln!(stream)?;
            writeln!(stream, "% ====   Song   ====")?;
            writeln!(stream, r"\newenvironment{{song}}[1]%")?;
            writeln!(stream, r"{{%")?;
            writeln!(
                stream,
                r"    \begin{{minipage}}[t]{{0.94\columnwidth}}{{\stepcounter{{songcount}}\textbf{{\large #1}}\index{{#1}}}}%"
            )?;
            writeln!(stream, r"        \par\vspace{{2pt}}")?;
            writeln!(stream, r"}}%")?;
            writeln!(stream, r"{{%")?;
            writeln!(stream, r"    \end{{minipage}}%")?;
            writeln!(stream, r"    \vspace{{2em}}%")?;
            writeln!(stream, r"}}")?;

            // Psalm environment
            writeln!(stream)?;
            writeln!(stream, "% ====   Psalm   ====")?;
            writeln!(stream, r"\newenvironment{{psalm}}[2]%")?;
            writeln!(stream, r"{{%")?;
            writeln!(stream, r"    \begin{{minipage}}[t]{{0.94\columnwidth}}%")?;
            writeln!(
                stream,
                r"        \begin{{center}}{{\stepcounter{{psalmcount}}\textbf{{\large #1}}\index{{#1}}{{\normalsize #2}}}}%"
            )?;
            writeln!(stream, r"            \par\vspace{{2pt}}")?;
            writeln!(stream, r"}}%")?;
            writeln!(stream, r"{{%")?;
            writeln!(stream, r"        \end{{center}}%")?;
            writeln!(stream, r"    \end{{minipage}}%")?;
            writeln!(stream, r"    \vspace{{2em}}%")?;
            writeln!(stream, r"}}%")?;

            // Utility commands
            writeln!(stream)?;
            writeln!(stream, "% ====   Utitilty Commands   ====")?;
            writeln!(
                stream,
                r"\newcommand{{\extra}}[1]{{\textit{{\normalsize (#1)}}}}"
            )?;
            writeln!(
                stream,
                r"\renewcommand{{\sp}}{{\textit{{\normalsize (Sing Psalms)}}}}"
            )?;
            writeln!(
                stream,
                r"\newcommand{{\tr}}{{\textit{{\normalsize (Scottish Psalter)}}}}"
            )?;
            writeln!(stream, r"\newcommand{{\LORD}}{{\textsc{{Lord}}}}")?;
            writeln!(stream, r"\newcommand{{\cp}}[1]{{{{\tiny\ttfamily#1}}}}")?;

            // Preamble extras
            if let Some(p) = &self.preamble_extra {
                writeln!(stream)?;
                writeln!(stream, "% ====   Rest of Preamble   ====")?;
                writeln!(stream, "{}", p)?;
            }

            // Document
            writeln!(stream)?;
            writeln!(stream, "% ====   Document   ====")?;
            writeln!(stream, r"\begin{{document}}")?;
            writeln!(stream, r"\sffamily")?;
            writeln!(stream)?;
            writeln!(stream, r"\begin{{titlepage}}")?;
            writeln!(stream, "{}", self.cover)?;
            writeln!(stream, r"\end{{titlepage}}")?;
            writeln!(stream)?;
            writeln!(
                stream,
                r"\setcounter{{page}}{{2}}  % Make title page, page 1"
            )?;
            writeln!(stream, r"\printindex")?;
            writeln!(stream, r"\begin{{multicols}}{{2}}")?;
            writeln!(stream, r"\raggedcolumns{{}}")?;

            // Body
            for s in &self.songs {
                writeln!(stream)?;
                writeln!(stream, "% ====   {}   ====", s.title)?;
                writeln!(stream, r"\begin{{song}}{{{}}}", Self::safe(&s.title))?;
                // Write verses, chorus and brides
                // Construction of the song checks order is valid
                let mut cur_verse: usize = 0;
                for c in s.order.chars() {
                    match c {
                        'v' => {
                            // Write current verse
                            writeln!(stream, r"    \verse")?;
                            for line in s
                                .verses
                                .get(cur_verse)
                                .ok_or(LaTeXError::VerseOutOfBounds {
                                    index: cur_verse,
                                    size: s.verses.len(),
                                })?
                                .lines()
                            {
                                writeln!(stream, "    {{{}}}", Self::safe(line))?;
                            }
                            writeln!(stream, r"    \end")?;
                            cur_verse += 1;
                        }
                        'c' => {
                            if let Some(chorus) = &s.chorus {
                                writeln!(stream, r"    \chorus")?;
                                for line in chorus.lines() {
                                    writeln!(stream, "    {{{}}}", Self::safe(line))?;
                                }
                                writeln!(stream, r"    \end")?;
                            }
                        }
                        'b' => {
                            if let Some(bridge) = &s.bridge {
                                writeln!(stream, r"    \bridge")?;
                                for line in bridge.lines() {
                                    writeln!(stream, "    {{{}}}", Self::safe(line))?;
                                }
                                writeln!(stream, r"    \end")?;
                            }
                        }
                        _ => {}
                    }
                }
                writeln!(stream, r"\end{{song}}")?;
            }

            // Finish up
            writeln!(stream)?;
            writeln!(stream, r"\end{{multicols}}")?;
            writeln!(stream, r"\end{{document}}")?;

            // Flush!
            stream.flush()?;
            // Don't need stream any more
        }

        Ok(LaTeX {
            path: self.path,
            config: self.config,
        })
    }

    pub fn use_package(mut self, pkg: Package) -> Self {
        self.packages.push(pkg);
        self
    }

    // Try and use const expressions to have minimum array length
    pub fn use_package_str(mut self, args: &[&str]) -> Result<Self, LaTeXError> {
        self.packages.push(args.try_into()?);
        Ok(self)
    }

    pub fn set_doc_class(mut self, doc_class: String) -> Self {
        self.doc_class = doc_class;
        self
    }

    pub fn set_doc_opts(mut self, doc_opts: Vec<String>) -> Self {
        self.doc_opts = doc_opts;
        self
    }

    pub fn set_version(mut self, version: (u8, u8, u8)) -> Self {
        self.version = version;
        self
    }

    pub fn set_verse_fmt(mut self, verse_fmt: String) -> Self {
        self.verse_fmt = verse_fmt;
        self
    }

    pub fn set_chorus_fmt(mut self, chorus_fmt: String) -> Self {
        self.chorus_fmt = chorus_fmt;
        self
    }

    pub fn set_bridge_fmt(mut self, bridge_fmt: String) -> Self {
        self.bridge_fmt = bridge_fmt;
        self
    }

    pub fn set_cover(mut self, cover: String) -> Self {
        self.cover = cover;
        self
    }

    pub fn set_preamble_extra(mut self, preamble_extra: Option<String>) -> Self {
        self.preamble_extra = preamble_extra;
        self
    }

    pub fn add_song(mut self, song: Song) -> Self {
        self.songs.push(song);
        self
    }

    fn safe<T>(string: T) -> String
    where
        T: AsRef<str>,
    {
        string
            .as_ref()
            .replace('%', r#"\%"#)
            .replace('$', r#"\$"#)
            .replace('{', r#"\{"#)
            .replace('}', r#"\}"#)
            .replace('#', r#"\#"#)
            .replace('&', r#"\&"#)
    }
}
