use crate::versuch::Versuch;

#[derive(Debug)]
pub struct Spielbrett {
    pub anzahl_versuche: u8,
    pub max_anzahl_versuche: u8,
    versuche: Vec<Versuch>,
}

impl Spielbrett {
    pub fn new(max_anzahl_versuche: u8) -> Self {
        Self {
            anzahl_versuche: 0u8,
            max_anzahl_versuche,
            versuche: vec![],
        }
    }

    pub fn protokolliere_versuch(&mut self, versuch: Versuch) {
        self.versuche.push(versuch);
    }

    pub fn liefere_versuche(&self) -> String {
        let mut parsed_versuche = String::from("VersuchNr:\tRatezahl:\tdirekte:\tindirekte:\t\n");

        for (idx, v) in self.versuche.iter().enumerate() {
            parsed_versuche.push_str(&format!(
                "\t{}\t{}\t\t{}\t\t{}\n",
                idx + 1,
                v.ratezahl,
                v.direkte_treffer,
                v.indirekte_treffer
            ));
        }

        parsed_versuche
    }

    pub fn ruecksetzen_versuche(&mut self) {
        self.anzahl_versuche = 0;
        self.versuche.clear();
    }
}
