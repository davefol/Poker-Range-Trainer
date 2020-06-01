use std::fmt;
use std::slice::Iter;
use std::ops::Sub;
use rand::{
    distributions::{Distribution, Standard},
    Rng
};
use serde::{Serialize, Deserialize};
use crate::svg;

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

#[derive(Copy, Clone)]
pub enum DisplaySuit {
    Clubs,
    Diamonds,
    Spades,
    Hearts
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

    pub fn svg_bytes(&self, display_suit: Option<DisplaySuit> ) -> &'static [u8] {
        match self {
            Card::Two => {
                match display_suit {
                    Some(DisplaySuit::Hearts) => svg::TWO_HEARTS,
                    Some(DisplaySuit::Clubs) => svg::TWO_CLUBS,
                    Some(DisplaySuit::Spades) => svg::TWO_SPADES,
                    Some(DisplaySuit::Diamonds) => svg::TWO_DIAMONDS,
                    None => {
                        [svg::TWO_HEARTS, svg::TWO_CLUBS, svg::TWO_SPADES, svg::TWO_DIAMONDS][rand::thread_rng().gen_range(0,4) as usize]
                    }
                }
            },


            Card::Three => {
                match display_suit {
                    Some(DisplaySuit::Hearts) => svg::THREE_HEARTS,
                    Some(DisplaySuit::Clubs) => svg::THREE_CLUBS,
                    Some(DisplaySuit::Spades) => svg::THREE_SPADES,
                    Some(DisplaySuit::Diamonds) => svg::THREE_DIAMONDS,
                    None => {
                        [svg::THREE_HEARTS, svg::THREE_CLUBS, svg::THREE_SPADES, svg::THREE_DIAMONDS][rand::thread_rng().gen_range(0,4) as usize]
                    }
                }
            },


            Card::Four => {
                match display_suit {
                    Some(DisplaySuit::Hearts) => svg::FOUR_HEARTS,
                    Some(DisplaySuit::Clubs) => svg::FOUR_CLUBS,
                    Some(DisplaySuit::Spades) => svg::FOUR_SPADES,
                    Some(DisplaySuit::Diamonds) => svg::FOUR_DIAMONDS,
                    None => {
                        [svg::FOUR_HEARTS, svg::FOUR_CLUBS, svg::FOUR_SPADES, svg::FOUR_DIAMONDS][rand::thread_rng().gen_range(0,4) as usize]
                    }
                }
            },


            Card::Five => {
                match display_suit {
                    Some(DisplaySuit::Hearts) => svg::FIVE_HEARTS,
                    Some(DisplaySuit::Clubs) => svg::FIVE_CLUBS,
                    Some(DisplaySuit::Spades) => svg::FIVE_SPADES,
                    Some(DisplaySuit::Diamonds) => svg::FIVE_DIAMONDS,
                    None => {
                        [svg::FIVE_HEARTS, svg::FIVE_CLUBS, svg::FIVE_SPADES, svg::FIVE_DIAMONDS][rand::thread_rng().gen_range(0,4) as usize]
                    }
                }
            },


            Card::Six => {
                match display_suit {
                    Some(DisplaySuit::Hearts) => svg::SIX_HEARTS,
                    Some(DisplaySuit::Clubs) => svg::SIX_CLUBS,
                    Some(DisplaySuit::Spades) => svg::SIX_SPADES,
                    Some(DisplaySuit::Diamonds) => svg::SIX_DIAMONDS,
                    None => {
                        [svg::SIX_HEARTS, svg::SIX_CLUBS, svg::SIX_SPADES, svg::SIX_DIAMONDS][rand::thread_rng().gen_range(0,4) as usize]
                    }
                }
            },


            Card::Seven => {
                match display_suit {
                    Some(DisplaySuit::Hearts) => svg::SEVEN_HEARTS,
                    Some(DisplaySuit::Clubs) => svg::SEVEN_CLUBS,
                    Some(DisplaySuit::Spades) => svg::SEVEN_SPADES,
                    Some(DisplaySuit::Diamonds) => svg::SEVEN_DIAMONDS,
                    None => {
                        [svg::SEVEN_HEARTS, svg::SEVEN_CLUBS, svg::SEVEN_SPADES, svg::SEVEN_DIAMONDS][rand::thread_rng().gen_range(0,4) as usize]
                    }
                }
            },


            Card::Eight => {
                match display_suit {
                    Some(DisplaySuit::Hearts) => svg::EIGHT_HEARTS,
                    Some(DisplaySuit::Clubs) => svg::EIGHT_CLUBS,
                    Some(DisplaySuit::Spades) => svg::EIGHT_SPADES,
                    Some(DisplaySuit::Diamonds) => svg::EIGHT_DIAMONDS,
                    None => {
                        [svg::EIGHT_HEARTS, svg::EIGHT_CLUBS, svg::EIGHT_SPADES, svg::EIGHT_DIAMONDS][rand::thread_rng().gen_range(0,4) as usize]
                    }
                }
            },


            Card::Nine => {
                match display_suit {
                    Some(DisplaySuit::Hearts) => svg::NINE_HEARTS,
                    Some(DisplaySuit::Clubs) => svg::NINE_CLUBS,
                    Some(DisplaySuit::Spades) => svg::NINE_SPADES,
                    Some(DisplaySuit::Diamonds) => svg::NINE_DIAMONDS,
                    None => {
                        [svg::NINE_HEARTS, svg::NINE_CLUBS, svg::NINE_SPADES, svg::NINE_DIAMONDS][rand::thread_rng().gen_range(0,4) as usize]
                    }
                }
            },


            Card::Ten => {
                match display_suit {
                    Some(DisplaySuit::Hearts) => svg::TEN_HEARTS,
                    Some(DisplaySuit::Clubs) => svg::TEN_CLUBS,
                    Some(DisplaySuit::Spades) => svg::TEN_SPADES,
                    Some(DisplaySuit::Diamonds) => svg::TEN_DIAMONDS,
                    None => {
                        [svg::TEN_HEARTS, svg::TEN_CLUBS, svg::TEN_SPADES, svg::TEN_DIAMONDS][rand::thread_rng().gen_range(0,4) as usize]
                    }
                }
            },


            Card::Jack => {
                match display_suit {
                    Some(DisplaySuit::Hearts) => svg::JACK_HEARTS,
                    Some(DisplaySuit::Clubs) => svg::JACK_CLUBS,
                    Some(DisplaySuit::Spades) => svg::JACK_SPADES,
                    Some(DisplaySuit::Diamonds) => svg::JACK_DIAMONDS,
                    None => {
                        [svg::JACK_HEARTS, svg::JACK_CLUBS, svg::JACK_SPADES, svg::JACK_DIAMONDS][rand::thread_rng().gen_range(0,4) as usize]
                    }
                }
            },


            Card::Queen => {
                match display_suit {
                    Some(DisplaySuit::Hearts) => svg::QUEEN_HEARTS,
                    Some(DisplaySuit::Clubs) => svg::QUEEN_CLUBS,
                    Some(DisplaySuit::Spades) => svg::QUEEN_SPADES,
                    Some(DisplaySuit::Diamonds) => svg::QUEEN_DIAMONDS,
                    None => {
                        [svg::QUEEN_HEARTS, svg::QUEEN_CLUBS, svg::QUEEN_SPADES, svg::QUEEN_DIAMONDS][rand::thread_rng().gen_range(0,4) as usize]
                    }
                }
            },


            Card::King => {
                match display_suit {
                    Some(DisplaySuit::Hearts) => svg::KING_HEARTS,
                    Some(DisplaySuit::Clubs) => svg::KING_CLUBS,
                    Some(DisplaySuit::Spades) => svg::KING_SPADES,
                    Some(DisplaySuit::Diamonds) => svg::KING_DIAMONDS,
                    None => {
                        [svg::KING_HEARTS, svg::KING_CLUBS, svg::KING_SPADES, svg::KING_DIAMONDS][rand::thread_rng().gen_range(0,4) as usize]
                    }
                }
            },


            Card::Ace => {
                match display_suit {
                    Some(DisplaySuit::Hearts) => svg::ACE_HEARTS,
                    Some(DisplaySuit::Clubs) => svg::ACE_CLUBS,
                    Some(DisplaySuit::Spades) => svg::ACE_SPADES,
                    Some(DisplaySuit::Diamonds) => svg::ACE_DIAMONDS,
                    None => {
                        [svg::ACE_HEARTS, svg::ACE_CLUBS, svg::ACE_SPADES, svg::ACE_DIAMONDS][rand::thread_rng().gen_range(0,4) as usize]
                    }
                }
            },
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

