#[derive(Debug, PartialEq)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand {
            cards: vec![],
        }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        let mut value:usize = 0;
        let mut high_aces:usize = 0;
        for card in self.cards.iter() {
            match card {
                Card::Ace if value > 10 => value += 1,
                Card::Ace if high_aces > 0 && value > 10 => value -= 10,
                Card::Ace => {
                    value += 11;
                    high_aces += 1;
                },
                Card::Two => value += 2,
                Card::Three => value += 3,
                Card::Four => value += 4,
                Card::Five => value += 5,
                Card::Six => value += 6,
                Card::Seven => value += 7,
                Card::Eight => value += 8,
                Card::Nine => value += 9,
                Card::Jack => value += 10,
                Card::Queen => value += 10,
                Card::King => value += 10,
            }
        }
        if value > 21{
            while  high_aces > 0 {
                high_aces -= 1;
                value -= 10;
            }
        }
        value
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Ace);
}


#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn double_aces_and_queen() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 12);
}

#[test]
fn double_aces_and_double_kings() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::King);
    hand.add(Card::Ace);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 22);
    assert_eq!(hand.is_loosing_hand(), true);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Ace);
    
    assert_eq!(hand.value(), 21);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Five);
    
    assert!(hand.is_loosing_hand());
    assert_eq!(hand.value(), 22);
}
