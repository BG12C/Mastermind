use rand::Rng;
use std::{char, collections::HashSet};

use crate::versuch::Versuch;

#[derive(Debug)]
pub struct Aufgabenersteller {
    pub geheimzahl: u16,
}

impl Aufgabenersteller {
    pub fn erzeuge_geheimzahl() -> Self {
        let mut rng = rand::thread_rng();

        loop {
            let geheimzahl = rng.gen_range(1000..10000);
            let digits = geheimzahl.to_string().chars().collect::<HashSet<char>>();
            if digits.len() == 4 {
                return Self { geheimzahl };
            }
        }
    }

    pub fn zerlege_zahl(zahl: u16) -> Vec<u8> {
        let stringified_number = zahl.to_string();
        let bytes = stringified_number.as_bytes();
        let digits: Vec<u8> = bytes
            .iter()
            .map(|x| {
                u8::try_from(char::try_from(*x).unwrap().to_digit(10).unwrap_or(0)).unwrap_or(0)
            })
            .collect();
        digits
    }

    pub fn einmaliger_inhalt(vec: &Vec<u8>) -> bool {
        let set: HashSet<_> = vec.iter().copied().collect();
        set.len() == vec.len()
    }

    pub fn bewerte_ratezahl(&self, ratezahl: u16) -> Versuch {
        let mut indirekte_treffer: u8 = 0;
        let mut direkte_treffer: u8 = 0;

        for (guess_digit_index, guess_digit) in ratezahl.to_string().chars().enumerate() {
            for (secret_digit_index, secret_digit) in
                self.geheimzahl.to_string().chars().enumerate()
            {
                if guess_digit == secret_digit {
                    if guess_digit_index == secret_digit_index {
                        direkte_treffer += 1;
                    } else {
                        indirekte_treffer += 1;
                    }
                }
            }
        }

        Versuch::new(ratezahl, direkte_treffer, indirekte_treffer)
    }
}
