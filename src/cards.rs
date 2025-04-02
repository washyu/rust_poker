use rand::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub fn to_string(&self) -> String {
        match self {
            Suit::Hearts => "hearts".to_string(),
            Suit::Diamonds => "diamonds".to_string(),
            Suit::Clubs => "clubs".to_string(),
            Suit::Spades => "spades".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub enum Rank {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King, Ace,
}

impl Rank {
    pub fn to_string(&self) -> String {
        match self {
            Rank::Two => "02".to_string(),
            Rank::Three => "03".to_string(),
            Rank::Four => "04".to_string(),
            Rank::Five => "05".to_string(),
            Rank::Six => "06".to_string(),
            Rank::Seven => "07".to_string(),
            Rank::Eight => "08".to_string(),
            Rank::Nine => "09".to_string(),
            Rank::Ten => "10".to_string(),
            Rank::Jack => "jack".to_string(),
            Rank::Queen => "queen".to_string(),
            Rank::King => "king".to_string(),
            Rank::Ace => "ace".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub back: i32
}

impl Card {
    pub fn image_filename(&self) -> String {
        format!("{}_{}.png",
            self.suit.to_string().to_lowercase(),
            self.rank.to_string())
    }

    pub fn back_image_filename(&self) -> String {
        format!("back{:02}.png", self.back) // Assuming the back of the card is a fixed image
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in [
                Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six,
                Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
                Rank::Jack, Rank::Queen, Rank::King, Rank::Ace
            ] {
                cards.push(Card { suit, rank, back: 1 });
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.deal()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn reset(&mut self) {
        *self = Deck::new();
        self.shuffle();
    }
}

pub fn evaluate_hand(hand: &[Card; 5]) -> HandRank {

    let is_flush = hand.windows(2).all(|w| w[0].suit == w[1].suit);

    let mut ranks: Vec<Rank> = hand.iter().map(|card| card.rank).collect();
    ranks.sort();

    let is_straight = ranks.windows(2).all(|w| (w[1] as usize) == (w[0] as usize) + 1);

    let mut rank_counts = std::collections::HashMap::new();
    for rank in ranks {
        *rank_counts.entry(rank).or_insert(0) += 1;
    }

    let max_count = *rank_counts.values().max().unwrap_or(&0);
    let pairs = rank_counts.values().filter(|&&count| count == 2).count();

    match (is_flush, is_straight, max_count, pairs) {
        (true, true, _, _) => HandRank::StraightFlush,
        (_, _, 4, _) => HandRank::FourOfAKind,
        (_, _, 3, 1) => HandRank::FullHouse,
        (true, _, _, _,) => HandRank::Flush,
        (_, true, _, _) => HandRank::Straight,
        (_, _, 3, _) => HandRank::ThreeOfAKind,
        (_, _, _, 2) => HandRank::TwoPair,
        (_, _, _, 1) => HandRank::OnePair,
        _ => HandRank::HighCard,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush
}

impl HandRank {
    pub fn to_string(&self) -> &'static str {
        match self {
            HandRank::HighCard => "High Card",
            HandRank::OnePair => "One Pair",
            HandRank::TwoPair => "Two Pair",
            HandRank::ThreeOfAKind => "Three of a Kind",
            HandRank::Straight => "Straight",
            HandRank::Flush => "Flush",
            HandRank::FullHouse => "Full House",
            HandRank::FourOfAKind => "Four of a Kind",
            HandRank::StraightFlush => "Straight Flush",
        }
    }

    pub fn payout(&self) -> i32 {
        match self {
            HandRank::HighCard => 0,
            HandRank::OnePair => 1,
            HandRank::TwoPair => 2,
            HandRank::ThreeOfAKind => 3,
            HandRank::Straight => 4,
            HandRank::Flush => 6,
            HandRank::FullHouse => 9,
            HandRank::FourOfAKind => 25,
            HandRank::StraightFlush => 50,
        }
    }
}