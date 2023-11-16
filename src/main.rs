use mastermind_ui::MastermindUI;

mod aufgabenersteller;
mod mastermind_ui;
mod spielbrett;
mod spieler;
mod versuch;

fn main() {
    let mut mastermind = MastermindUI::new(10);
    while mastermind.neues_spiel().eq(&1) {
        mastermind.neues_spiel();
    }

    println!("******************** Danke und Tschüss!!! ********************");
}
