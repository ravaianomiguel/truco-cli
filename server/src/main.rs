fn main() {
    let deck = gen_deck();
    for card in deck{
        println!("{card:?}")
    }
}
#[derive(Debug)]
struct Card{
    card: String,
    value: u8
}
fn gen_deck() -> Vec<Card>{
    let mut deck: Vec<Card> = Vec::new();
    let suits: Vec<char> = vec!['♣','♦','♥','♠'];
    for n in 1..7 as u8{
        let num = {
            if n <= 4 {
                if n == 1{
                    'Q'
                } else if n == 2 {
                    'J'
                } else if n == 3{
                    'K'
                } else {
                    'A'
                }
            } else if n == 7 {
                '7'
            } else {
                (n-3) as char
            }
        };
        for s in suits.clone(){
            let val: u8 = {
                if num == 'A' && s == '♠'{
                    8
                } else if num == '7' && s == '♥' {
                    9
                } else {
                    n
                }
            };
            if !(num =='7' && (s  == '♣' || s == '♠')){
                deck.push(Card { card: format!("{num}{s}"), value: val })
            }
        }
    };
    deck.push(Card { card: String::from("4♣"), value: 10 });
    deck
}