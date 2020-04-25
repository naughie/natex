enum DocState {
    PAR,
}

pub struct Document {
    state: DocState,
}

impl Document {
    pub fn new(s: String) -> Result<Document, String> {
        Ok(Document {
            state: DocState::PAR,
        })
    }
}
