use std::io::{self, Write};

#[derive(Debug)]
pub struct Spieler;

impl Spieler {
    pub fn rate_zahl(verbleibend: u8) -> String {
        print!("Sie haben noch {verbleibend} Versuch(e): ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        buffer
    }
}
