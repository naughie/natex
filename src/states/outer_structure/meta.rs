pub struct Meta {
    pub title: String,
    pub author: String,
    pub affil: String,
    pub date: String,
}

impl Meta {
    pub fn new() -> Meta {
        Meta {
            title: String::from(""),
            author: String::from(""),
            affil: String::from(""),
            date: String::from(""),
        }
    }

    pub fn tran(self, s: String) -> Result<Meta, String> {
        let len = s.len();
        if len > 7 && &s[0..7] == "title: " {
            Ok(Meta {
                title: s[7..].to_string(),
                author: self.author,
                affil: self.affil,
                date: self.date,
            })
        } else if len > 8 && &s[0..8] == "author: " {
            Ok(Meta {
                title: self.title,
                author: s[8..].to_string(),
                affil: self.affil,
                date: self.date,
            })
        } else if len > 7 && &s[0..7] == "affil: " {
            Ok(Meta {
                title: self.title,
                author: self.author,
                affil: s[7..].to_string(),
                date: self.date,
            })
        } else if len > 6 && &s[0..6] == "date: " {
            Ok(Meta {
                title: self.title,
                author: self.author,
                affil: self.affil,
                date: s[6..].to_string(),
            })
        } else {
            Err(s)
        }
    }
}
