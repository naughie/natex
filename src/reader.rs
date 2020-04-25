use super::states::outer_structure::States;
use super::writer::Writer;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Reader<T: Writer> {
    o: T,
    b: BufReader<File>,
}

impl<T: Writer> Reader<T> {
    pub fn new(o: T, f: File) -> Reader<T> {
        Reader {
            o: o,
            b: BufReader::new(f),
        }
    }

    pub fn read(mut self) -> std::io::Result<()> {
        let mut s = States::START;
        for line in self.b.lines() {
            if line.is_err() {
                return line.map(|_| ());
            }
            match s.tran(line.unwrap(), &mut self.o) {
                Ok(t) => s = t,
                Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
            }
        }
        self.o.print_end_document();
        self.o.print_end();
        Ok(())
    }
}
