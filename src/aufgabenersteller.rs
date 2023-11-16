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

    // TODO: implement
    // pub fn zerlege_zahl(&self, zahl: u16) -> Vec<u8> {
    //     let stringified_number = zahl.to_string();
    //     let bytes = stringified_number.as_bytes();
    //     let digits: Vec<u8> = bytes
    //         .iter()
    //         .map(|x| char::try_from(*x).unwrap().to_digit(10).unwrap_or(0) as u8)
    //         .collect();
    //     digits
    // }

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
