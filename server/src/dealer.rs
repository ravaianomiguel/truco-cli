use crate::Card;
use promptly::prompt;
pub fn deal_cards(
    mut deck: Vec<Card>,
    mut table: Vec<Card>,
    mut hands: Vec<Vec<Card>>,
) -> (Vec<Card>, Vec<Card>, Vec<Vec<Card>>) {
    for hand in &mut hands {
        let opt: String = prompt("quer ou queima").unwrap();
        let mut transfer: Vec<Card> = Vec::new();
        if opt == "1" {
            (table, deck) = give_cards(table, deck)
        }
        (transfer, deck) = give_cards(transfer, deck);
        for card in transfer {
            print!("{}  ", card.card)
        }
        print!("\n");
        if prompt("familia").unwrap() {
            if familia(transfer) {
                (table, transfer) = give_cards(table, transfer);
                (hand, deck) = give_cards(hand, deck)
            } else {
                (hand, transfer) = give_cards(hand, transfer)
            }
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
fn familia(transfer: Vec<Card>) -> bool {
    for card in &transfer {
        if card.value > 3 {
            return false;
        }
    }
    true
}
fn give_cards(mut target: Vec<Card>, mut source: Vec<Card>) -> (Vec<card>, Vec<Card>) {
    for _i in 1..4 {
        target.push(source.pop().unwrap());
    }
    (target, source)
}
