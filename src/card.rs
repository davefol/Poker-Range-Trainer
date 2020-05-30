use std::fmt;
use std::slice::Iter;
use std::ops::Sub;
use rand::{
    distributions::{Distribution, Standard},
    Rng
};
use serde::{Serialize, Deserialize};

#[derive(
    PartialEq, Eq, Hash, Serialize, Deserialize, Ord, 
    PartialOrd, Copy, Clone, Debug
    )
]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, Ord, PartialOrd)]
pub enum Gap {
    Same,
    Connected,
    One,
    Two,
    Three,
    Large
}

impl Card {
    pub fn iterator() -> Iter<'static, Card> {
        use self::Card::*;
        static CARDS: [Card; 13] = [Two, 
        Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, 
        King, Ace];
        CARDS.iter()
    }

    pub fn chen_points(&self) -> f32 {
        match self {
            Card::Ace => 10.0,
            Card::King => 8.0,
            Card::Queen => 7.0,
            Card::Jack => 6.0,
            card => u8::from(*card) as f32 / 2.0
        }
    } 
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Card::Two => "2",
            Card::Three => "3",
            Card::Four => "4",
            Card::Five => "5",
            Card::Six => "6",
            Card::Seven => "7",
            Card::Eight => "8",
            Card::Nine => "9",
            Card::Ten => "T",
            Card::Jack => "J",
            Card::Queen => "Q",
            Card::King => "K",
            Card::Ace => "A"
        })
    }
}

impl Sub for Card {
    type Output = Gap;

    fn sub(self, other: Card) -> Gap {
        match i8::abs(u8::from(self) as i8 - u8::from(other) as i8) as u8 {
            0 => Gap::Same,
            1 => Gap::Connected,
            2 => Gap::One,
            3 => Gap::Two,
            4 => Gap::Three,
            _ => Gap::Large
        }
    }
}

impl From<Card> for u8 {
    fn from(card: Card) -> u8 {
        match card {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14
        }
    }
}

impl Distribution<Card> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        match rng.gen_range(2,15) {
            2 => Card::Two,
            3 => Card::Three,
            4 => Card::Four,
            5 => Card::Five,
            6 => Card::Six,
            7 => Card::Seven,
            8 => Card::Eight,
            9 => Card::Nine,
            10 => Card::Ten,
            11 => Card::Jack,
            12 => Card::Queen,
            13 => Card::King,
            _ => Card::Ace
        }
    }
}
