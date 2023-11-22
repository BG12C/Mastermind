use std::{
    io::{self, Write},
    ops::AddAssign,
};

use crate::{aufgabenersteller::Aufgabenersteller, spielbrett::Spielbrett, spieler::Spieler};

#[derive(Debug)]
pub struct MastermindUI {
    pub max_anzahl_versuche: u8,
    pub aufgabenersteller: Aufgabenersteller,
    pub spielbrett: Spielbrett,
}

impl MastermindUI {
    pub fn new(max_anzahl: u8) -> Self {
        MastermindUI {
            max_anzahl_versuche: max_anzahl,
            aufgabenersteller: Aufgabenersteller::erzeuge_geheimzahl(),
            spielbrett: Spielbrett::new(),
        }
    }

    pub fn neues_spiel(&mut self) -> u8 {
        self.aufgabenersteller = Aufgabenersteller::erzeuge_geheimzahl();
        self.spielbrett.ruecksetzen_versuche();

        loop {
            if self.spielbrett.anzahl_versuche >= self.max_anzahl_versuche {
                println!("Sie konnten die Geheimzahl nicht innerhalb von 10 Zügen herausfinden!\nDie Geheimzahl war {}", self.aufgabenersteller.geheimzahl);
                break;
            }

            let input =
                Spieler::rate_zahl(self.max_anzahl_versuche - self.spielbrett.anzahl_versuche);
            let trimmed_input = input.trim();

            let rate_zahl = if let Ok(n) = trimmed_input.parse::<u16>() {
                if (1000..10000).contains(&n) {
                    if Aufgabenersteller::einmaliger_inhalt(&Aufgabenersteller::zerlege_zahl(n)) {
                        n
                    } else {
                        println!("Jede Zahl kann nur einmal vorkommen!");
                        continue;
                    }
                } else {
                    println!("{trimmed_input} ist zu groß oder klein! Die Zahl muss zwischen 1000 & 9999 liegen!");
                    continue;
                }
            } else {
                println!("'{trimmed_input}' ist keine gültige Zahl!");
                continue;
            };

            let versuch = self.aufgabenersteller.bewerte_ratezahl(rate_zahl);

            self.spielbrett.protokolliere_versuch(versuch);
            println!("{}", self.spielbrett.liefere_versuche());
            self.spielbrett.anzahl_versuche.add_assign(1);

            if versuch.direkte_treffer.eq(&4) {
                println!(
                    "Volltreffer!!! Sie haben die Zahl im {}. Versuch erraten.",
                    self.spielbrett.anzahl_versuche
                );
                break;
            }
        }

        print!("Möchten sie noch einmal spielen? (1=JA / 2=NEIN): ");
        io::stdout().flush().unwrap();
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();

            if let Ok(n) = buffer.trim().parse::<u8>() {
                if n == 1 || n == 2 {
                    return n;
                }
            }
        }
    }
}
