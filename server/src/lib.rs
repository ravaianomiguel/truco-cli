use promptly::prompt;
pub fn get_player_count() -> Vec<Vec<Card>> {
    let mut hands: Vec<Vec<Card>> = Vec::new();
    let playas: i32 = {
        let p: bool = prompt("4p").unwrap();
        if p {
            4
        } else {
            2
        }
    };
    for _i in 0..playas {
        hands.push(Vec::new());
    }
    hands
}
#[derive(Debug, Clone)]
pub struct Card {
    pub card: String,
    pub value: u8,
}
