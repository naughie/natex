pub mod document;
pub mod meta;

use self::document::Document;
use self::meta::Meta;

use super::super::writer::Writer;

pub enum States {
    START,
    META(Meta),
    DOCUMENT(Document),
}

impl States {
    pub fn tran<T: Writer>(self, s: String, o: &mut T) -> Result<States, String> {
        match self {
            Self::START => Self::tran_start(s, o),
            Self::META(meta) => Self::tran_meta(meta, s, o),
            Self::DOCUMENT(doc) => Self::tran_document(doc, s, o),
        }
    }

    fn tran_start<T: Writer>(s: String, o: &mut T) -> Result<States, String> {
        if s == "---" {
            Ok(Self::META(Meta::new()))
        } else {
            Document::new(s).map(|d| Self::DOCUMENT(d))
        }
    }

    fn tran_meta<T: Writer>(meta: Meta, s: String, o: &mut T) -> Result<States, String> {
        if s == "---" {
            o.print_meta(meta);
            Document::new("".to_string()).map(|d| Self::DOCUMENT(d))
        } else {
            match meta.tran(s) {
                Ok(m) => Ok(Self::META(m)),
                Err(s) => Document::new(s).map(|d| Self::DOCUMENT(d)),
            }
        }
    }

    fn tran_document<T: Writer>(doc: Document, s: String, o: &mut T) -> Result<States, String> {
        Document::new(s).map(|d| Self::DOCUMENT(d))
    }
}
