use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use itertools::Itertools;
use crate::hand::{Hand, Suit};
use crate::card::Card;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Range {
    pub name: String,
    pub hands: HashSet<Hand>
}

impl Range {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_percent(percent: u8) -> Self {
        let first_cards = Card::iterator();
        let second_cards = Card::iterator();
        let suits = [Suit::Suited, Suit::Off].iter();


        let mut all_cards = suits
            .cartesian_product(first_cards.cartesian_product(second_cards))
            .into_iter()
            .map(|(suited, (first, second))| {
                Hand { first: *first, second: *second, suited: *suited }
            })
            .filter(|hand| {
                !(hand.first == hand.second && hand.suited == Suit::Suited)        
            })
            .collect::<Vec<Hand>>();
        
        all_cards.sort_unstable_by(|a, b| {
            a.chen_value().partial_cmp(&b.chen_value()).unwrap()
        });

        Range {
            name: format!("Top {} percent", percent),
            hands: all_cards
                .iter()
                .rev()
                .take(all_cards.len() * percent as usize / 100_usize)
                .map(|x| *x)
                .collect::<HashSet<Hand>>()
        }
    }

    pub fn toggle(&mut self, hand: Hand) {
        if self.hands.contains(&hand) {
            self.hands.remove(&hand);
        } else {
            self.hands.insert(hand);
        }
    }

    pub fn contains(&self, hand: &Hand) -> bool {
        self.hands.contains(hand)
    }
}

impl Default for Range {
    fn default() -> Range {
        Range {
            name: String::from("untitled range"),
            hands: HashSet::<Hand>::new(),
        }
    }
}
