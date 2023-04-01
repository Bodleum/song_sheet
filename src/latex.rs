use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::Song;

/// Represents a LaTeX package
#[derive(Debug, Default)]
pub struct Package {
    pub name: String,
    pub opts: Option<Vec<String>>,
}

impl From<&[&str]> for Package {
    fn from(args: &[&str]) -> Self {
        let name = args
            .into_iter()
            .next()
            .expect("Must give package name!") // Can unwrap if array has guaranteed min length
            .to_string();

        let opts: Vec<String> = args
            .into_iter()
            .skip(1)
            .map(|opt| opt.to_string())
            .collect();

        Package {
            name,
            opts: (opts.len() > 0).then_some(opts),
        }
    }
}

/// Represents a LaTeX file
#[derive(Debug)]
pub struct Latex {
    file: File,
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
}

impl Latex {
    pub fn new(file: File) -> Self {
        let mut i = Self {
            file,
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
            cover: String::from(r"\includepdf{./titleimage.jpg}"),
            preamble_extra: None,
            songs: Vec::<Song>::new(),
        };

        // Add default packages
        i.use_package_str(&["geometry", "left=1cm", "right=1cm", "top=1cm", "bottom=2cm"])
            .unwrap();
        i.use_package_str(&["hyperref", "hyperindex"]).unwrap();
        i.use_package_str(&["makeidx"]).unwrap();
        i.use_package_str(&["pdfpages"]).unwrap();
        i.use_package_str(&["fancyhdr"]).unwrap();
        i.use_package_str(&["graphicx"]).unwrap();
        i.use_package_str(&["adjustbox"]).unwrap();
        i.use_package_str(&["multicol"]).unwrap();
        i.use_package_str(&["totcount"]).unwrap();
        i.use_package_str(&["xcolor"]).unwrap();

        i
    }

    pub fn use_package(&mut self, pkg: Package) {
        self.packages.push(pkg);
    }

    // Try and use const expressions to have minimum array length
    pub fn use_package_str(&mut self, args: &[&str]) -> Result<(), &str> {
        if args.len() == 0 {
            return Err("Must provide package name!");
        }
        self.packages.push(args.into());
        Ok(())
    }

    pub fn write_to_file(&self) -> std::io::Result<()> {
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
            writeln!(stream, "% ====   {}   ====", s.name)?;
            writeln!(stream, r"\begin{{song}}{{{}}}", s.name)?;
            // Write verses, chorus and brides
            // Construction of the song checks order is valid
            let mut cur_verse: usize = 0;
            for c in s.order.chars() {
                match c {
                    'v' => {
                        // Write current verse
                        writeln!(stream, r"    \verse")?;
                        writeln!(stream, "{}", s.verses[cur_verse])?;
                        writeln!(stream, r"    \end")?;
                        cur_verse += 1;
                    }
                    'c' => {
                        if let Some(chorus) = &s.chorus {
                            writeln!(stream, r"    \chorus")?;
                            writeln!(stream, "{}", chorus)?;
                            writeln!(stream, r"    \end")?;
                        }
                    }
                    'b' => {
                        if let Some(bridge) = &s.bridge {
                            writeln!(stream, r"    \bridge")?;
                            writeln!(stream, "{}", bridge)?;
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
        Ok(())
    }

    pub fn set_doc_class(&mut self, doc_class: String) {
        self.doc_class = doc_class;
    }

    pub fn set_doc_opts(&mut self, doc_opts: Vec<String>) {
        self.doc_opts = doc_opts;
    }

    pub fn set_version(&mut self, version: (u8, u8, u8)) {
        self.version = version;
    }

    pub fn set_verse_fmt(&mut self, verse_fmt: String) {
        self.verse_fmt = verse_fmt;
    }

    pub fn set_chorus_fmt(&mut self, chorus_fmt: String) {
        self.chorus_fmt = chorus_fmt;
    }

    pub fn set_bridge_fmt(&mut self, bridge_fmt: String) {
        self.bridge_fmt = bridge_fmt;
    }

    pub fn set_cover(&mut self, cover: String) {
        self.cover = cover;
    }

    pub fn set_preamble_extra(&mut self, preamble_extra: Option<String>) {
        self.preamble_extra = preamble_extra;
    }

    pub fn add_song(&mut self, song: Song) {
        self.songs.push(song);
    }
}