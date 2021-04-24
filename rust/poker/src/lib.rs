#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum PokerHand {
    HighCard(u8, u8, u8, u8, u8),
    OnePair(u8, u8, u8, u8),
    TwoPair(u8, u8, u8),
    ThreeOfAKind(u8, u8, u8),
    Straight(u8),
    Flush(u8, u8, u8, u8, u8),
    FullHouse(u8, u8),
    FourOfAKind(u8, u8),
    StraightFlush(u8)
}

impl PokerHand {
    fn new(cards: &mut [Card]) -> Self {
        cards.sort_by(|x, y| y.cmp(x));

        let straight = cards.windows(2).all(|w| w[0].0 == w[1].0 + 1 || w[0].0 == 14 && w[1].0 == 5);
        let flush = cards.iter().all(|card| card.1 == cards[0].1);

        let mut ranks = (0..=14).map(|i| {
            (cards.iter().filter(|Card (rank, _)| *rank == i).count() as u8, i)
        }).filter(|(i, _)| *i != 0).collect::<Vec<_>>();
        ranks.sort_by(|x, y| y.cmp(x));
        let ranks = ranks.as_slice();

        if straight && flush {
            PokerHand::StraightFlush(cards[4].0)
        } else if ranks[0].0 == 4 {
            PokerHand::FourOfAKind(ranks[0].1, ranks[1].1)
        } else if ranks[0].0 == 3 && ranks[1].0 == 2 {
            PokerHand::FullHouse(ranks[0].1, ranks[1].1)
        } else if flush {
            PokerHand::Flush(cards[0].0, cards[1].0, cards[2].0, cards[3].0, cards[4].0)
        } else if straight {
            if cards[0].0 == 14 {
                PokerHand::Straight(5)
            } else {
                PokerHand::Straight(cards[0].0)
            }
        } else if ranks[0].0 == 3 {
            PokerHand::ThreeOfAKind(ranks[0].1,
                                    ranks[1].1,
                                    ranks[2].1)
        } else if ranks[0].0 == 2 {
            if ranks[1].0 == 2 {
                PokerHand::TwoPair(ranks[0].1,
                                    ranks[1].1,
                                    ranks[2].1)
            } else {
                PokerHand::OnePair(ranks[0].1,
                                    ranks[1].1,
                                    ranks[2].1,
                                    ranks[3].1)
            }
        } else {
            PokerHand::HighCard(cards[0].0, cards[1].0, cards[2].0, cards[3].0, cards[4].0)
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
struct Card(u8, char);

impl Card {
    fn new(card_str: &str) -> Self {
        let rank: u8;
        if card_str.len() == 3 {
            rank = 10;
        } else {
            rank = match card_str.chars().nth(0).unwrap() {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                c => c as u8 - b'0'
            }
        }
        let suit = card_str.chars().last().unwrap();

        Card (rank,suit)
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut hands = hands.iter().map(|hand| {
        let pokerhand = PokerHand::new(hand.split( | c: char | c.is_whitespace())
            .map( | card| Card::new(card))
            .collect::<Vec<Card>>().as_mut_slice());

        (pokerhand, *hand)
    }).collect::<Vec<(PokerHand, &'a str)>>();

    hands.sort_by(|x, y| y.cmp(x));
    let winning = hands.iter().take_while(|hand| {
        hand.0.eq(&hands.iter().nth(0).unwrap().0)
    }).map(|h| h.1).collect();

    Some(winning)
}