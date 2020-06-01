use std::fmt;
use rand::{
    distributions::{Distribution, Standard},
    Rng
};
use serde::{Serialize, Deserialize};

use crate::card::{Card, Gap, DisplaySuit};

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Hand {
    pub first: Card,
    pub second: Card,
    pub suited: Suit
}

impl Hand {
    pub fn chen_value(&self) -> f32 {
        let mut points = Card::max(self.first, self.second).chen_points();

        if self.first == self.second {
           if self.first == Card::Five {
                points = 6.0;
           } else {
                points = f32::max(points * 2.0, 5.0);
           }
        } 

        if self.suited == Suit::Suited {
            points += 2.0;
        }

        let gap = self.first - self.second;
        points -= match gap {
            Gap::Same => 0.0,
            Gap::Connected => 0.0,
            Gap::One => 1.0,
            Gap::Two => 2.0,
            Gap::Three => 4.0,
            Gap::Large => 5.0
        };

        if gap > Gap::Same && gap <= Gap::One && Card::max(self.first, self.second) < Card::Queen {
            points += 1.0;
        }

        points
    }
}

impl From<Hand> for (&'static [u8], &'static[u8]) {
    fn from(hand: Hand) ->  (&'static [u8], &'static[u8] ){
        let mut suits = vec![DisplaySuit::Hearts, DisplaySuit::Clubs, DisplaySuit::Spades, DisplaySuit::Diamonds];
        let first_suit = suits.remove(rand::thread_rng().gen_range(0,4));
        let second_suit = match hand.suited {
            Suit::Suited => first_suit,
            Suit::Off => suits.remove(rand::thread_rng().gen_range(0,3))
        };
        let first_svg = hand.first.svg_bytes(Some(first_suit));
        let second_svg = hand.second.svg_bytes(Some(second_suit));
        (first_svg, second_svg)
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.first == self.second {
            write!(f, "{}{}", self.first, self.second)
        } else {
            write!(f, "{}{}{}", self.first, self.second, self.suited)
        }
    }
}

impl Distribution<Hand> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hand {
        let a: Card = rand::random();
        let b: Card = rand::random();
        let (first, second) = (Card::max(a,b), Card::min(a,b));
        let suited = {
            if first == second {
                Suit::Off
            } else {
                match rng.gen_range(0,2) {
                    0 => Suit::Off,
                    _ => Suit::Suited
                }
            }
        };
        Hand {first, second, suited}
    }
}


#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Suit {
    Suited,
    Off
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Suit::Suited => "s",
            Suit::Off => "o",
        })
    }
}
