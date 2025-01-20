use crate::deckgen::gen_deck;
mod deckgen;
fn main() {
    let deck = gen_deck();
    for card in deck {
        println!("{}", card.card)
    }
}
struct Card {
    card: String,
    value: u8,
}
