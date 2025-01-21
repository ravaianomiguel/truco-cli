use crate::Card;
use promptly::prompt;
pub fn deal_cards(
    mut deck: Vec<Card>,
    mut table: Vec<Card>,
    mut hands: Vec<Vec<Card>>,
) -> (Vec<Card>, Vec<Card>, Vec<Vec<Card>>) {
    for hand in &mut hands {
        let opt: String = prompt("quer ou queima").unwrap();
        if opt == "1" {
            for _i in 1..4 {
                table.push(deck.pop().unwrap())
            }
        }
        for _i in 1..4 {
            hand.push(deck.pop().unwrap());
        }
    }
    print_cards(&table, &hands);
    (deck, table, hands)
}
fn print_cards(table: &Vec<Card>, hands: &Vec<Vec<Card>>) {
    println!("cartas na mesa:");
    for card in table {
        print!("{}  ", card.card)
    }
    let mut ind = 1;
    for hand in hands {
        println!("\n cartas na mão {ind}");
        for card in hand {
            print!("{}  ", card.card)
        }
        ind += 1
    }
    print!("\n");
}
