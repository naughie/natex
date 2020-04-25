use std::fs::File;

mod reader;
mod states;
mod writer;

use self::reader::Reader;
use self::writer::latex::LaTeX;
use self::writer::Writer;

pub fn latex() -> LaTeX<std::io::Stdout> {
    LaTeX::new(std::io::stdout())
}

pub fn reader<T: Writer>(o: T, f: File) -> Reader<T> {
    Reader::new(o, f)
}
