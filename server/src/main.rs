use crate::dealer::deal_cards;
use crate::deckgen::gen_deck;
use truco_cli_server::get_player_count;
use truco_cli_server::Card;
mod dealer;
mod deckgen;
fn main() {
    let mut deck = gen_deck();
    let mut table: Vec<Card> = Vec::new();
    let mut hands = get_player_count();
    (deck, table, hands) = deal_cards(deck, table, hands);
}
