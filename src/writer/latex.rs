use super::super::states::outer_structure::meta::Meta;
use super::super::writer::Writer;

use std::io::{BufWriter, Write};

pub struct LaTeX<W: Write> {
    b: BufWriter<W>,
}

impl<W: Write> Writer for LaTeX<W> {
    fn print_meta(&mut self, meta: Meta) {
        write!(
            self.b,
            "\\documentclass[a4paper]{{article}}\n\
             \\title{{{}}}\n\
             \\author{{{}}}\n\
             \\affil{{{}}}\n\
             \\date{{{}}}\n\
             \\begin{{document}}\n",
            meta.title, meta.author, meta.affil, meta.date
        )
        .unwrap();
    }
}

impl<W: Write> LaTeX<W> {
    pub fn new(w: W) -> LaTeX<W> {
        LaTeX {
            b: BufWriter::new(w),
        }
    }
}
