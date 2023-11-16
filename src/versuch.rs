#[derive(Debug, Clone, Copy)]
pub struct Versuch {
    pub ratezahl: u16,
    pub direkte_treffer: u8,
    pub indirekte_treffer: u8,
}

impl Versuch {
    pub fn new(ratezahl: u16, direkte_treffer: u8, indirekte_treffer: u8) -> Self {
        Self {
            ratezahl,
            direkte_treffer,
            indirekte_treffer,
        }
    }
}
