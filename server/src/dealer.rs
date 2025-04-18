use crate::Card;
use promptly::prompt;
pub fn deal_cards(
    mut deck: Vec<Card>,
    mut table: Vec<Card>,
    mut hands: Vec<Vec<Card>>,
) -> (Vec<Card>, Vec<Card>, Vec<Vec<Card>>) {
    for hand in  &mut hands {
        let opt: String = prompt("quer ou queima").unwrap();
        let mut transfer: Vec<Card> = Vec::new();
        if opt == "1" {
            deck = table.recieve_cards(deck)
        }
        deck = transfer.recieve_cards(deck);
        for card in &transfer {
            print!("{}  ", card.card)
        }
        print!("\n");
        if prompt("familia").unwrap() {
            if familia(&transfer) {
                table.recieve_cards(transfer);
                deck = hand.recieve_cards(deck)
            } else {
                hand.recieve_cards(transfer);
            }
        } else {
            hand.recieve_cards(transfer);
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
fn familia(transfer: &Vec<Card>) -> bool {
    for card in transfer {
        if card.value > 4 && card.card != "A♤" {
            return false;
        }
    }
    true
}
trait RecieveCards {
    fn recieve_cards(&mut self, source: Vec<Card>) -> Vec<Card>;
}
impl RecieveCards for Vec<Card> {
    fn recieve_cards(&mut self, mut source: Vec<Card>) -> Vec<Card> {
        for _i in 1..4 {
            self.push(source.pop().unwrap());
        };
        source
    }
}