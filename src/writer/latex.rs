use super::super::states::outer_structure::meta::Meta;
use super::super::writer::Writer;

use std::io::{BufWriter, Write};

pub struct LaTeX<W: Write> {
    b: BufWriter<W>,
}

impl<W: Write> Writer for LaTeX<W> {
    fn print_begin(&mut self) {
        write!(self.b, "\\documentclass[a4paper]{{article}}\n").unwrap();
    }

    fn print_end(&mut self) {}

    fn print_meta(&mut self, meta: Meta) {
        write!(
            self.b,
            "\\title{{{}}}\n\
             \\author{{{}}}\n\
             \\affil{{{}}}\n\
             \\date{{{}}}\n",
            meta.title, meta.author, meta.affil, meta.date
        )
        .unwrap();
    }

    fn print_begin_document(&mut self) {
        write!(self.b, "\\begin{{document}}\n").unwrap();
    }

    fn print_end_document(&mut self) {
        write!(self.b, "\\end{{document}}\n").unwrap();
    }
}

impl<W: Write> LaTeX<W> {
    pub fn new(w: W) -> LaTeX<W> {
        LaTeX {
            b: BufWriter::new(w),
        }
    }
}
