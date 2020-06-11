use crate::range;
use crate::review;
use directories::ProjectDirs;
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::Write;

const TLD: &'static str = "com";
const ORG: &'static str = "Marble Devices";
const APP: &'static str = "Range Trainer";

#[derive(Clone, Debug, PartialEq)]
pub struct ReviewCollection(
    HashMap<(review::item::Presentation, review::item::Answer), review::item::ReviewItem>,
);

impl ReviewCollection {
    pub fn new() -> Self {
        Self(HashMap::<(review::item::Presentation, review::item::Answer), review::item::ReviewItem>::new())
    }

    pub fn get(&self, key: &(review::item::Presentation, review::item::Answer)) -> Option<&review::item::ReviewItem>{
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: &(review::item::Presentation, review::item::Answer)) -> Option<&mut review::item::ReviewItem>{
        self.0.get_mut(key)
    }

    pub fn insert(&mut self, key: (review::item::Presentation, review::item::Answer), value: review::item::ReviewItem) -> Option<review::item::ReviewItem> {
        self.0.insert(key, value)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<(review::item::Presentation, review::item::Answer), review::item::ReviewItem> {
        self.0.iter()
    }
}

impl Serialize for ReviewCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(self.0.iter().map(|(k, v)| (serialize_review_collection_key(k), v)))
    }
}

impl<'de> Deserialize<'de> for ReviewCollection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReviewCollectionVisitor;

        impl<'de> Visitor<'de> for ReviewCollectionVisitor {
            type Value = ReviewCollection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a ReviewCollection")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut review_collection = ReviewCollection(HashMap::<
                    (review::item::Presentation, review::item::Answer),
                    review::item::ReviewItem,
                >::with_capacity(
                    access.size_hint().unwrap_or(0)
                ));

                while let Some((key, value)) = access.next_entry::<String,review::item::ReviewItem>()? {
                    if let Some(key) = parse_review_collection_key(&key) {
                         review_collection.0.insert(key, value);
                    } else {
                        return Err(serde::de::Error::custom("failed parsing review collection key"));
                    }
                }

                Ok(review_collection)
            }
        }

        deserializer.deserialize_map(ReviewCollectionVisitor)
    }
}

fn parse_review_collection_key(s: &str) -> Option<(review::item::Presentation, review::item::Answer)> {
    let mut it = s.split('⦙');
    let maybe_presentation = it.next();
    let presentation = maybe_presentation.map(|x| x.parse::<review::item::Presentation>().ok()).unwrap_or(None);
    let maybe_answer = it.next();
    let answer = maybe_answer.map(|x| x.parse::<review::item::Answer>().ok()).unwrap_or(None);
    if presentation.is_none() {
        dbg!((&presentation, maybe_presentation));
    }
    if answer.is_none() {
        dbg!((&answer, maybe_answer));
    }
    if let (Some(presentation), Some(answer)) = (presentation, answer) {
        Some((presentation, answer))
    } else {
        None
    }
}

fn serialize_review_collection_key(k:&(review::item::Presentation, review::item::Answer)) -> String {
        format!("{}⦙{}", k.0, k.1).to_string()
}

pub fn load_review_items(
) -> ReviewCollection {
    if let Some(proj_dirs) = ProjectDirs::from(TLD, ORG, APP) {
        let data_dir = proj_dirs.data_dir().join("review_items.json");
        let path = data_dir.as_path();
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(data) = serde_json::from_str(&contents) {
                return data;
            }
        }
    }
    return ReviewCollection::new();
}

pub fn save_review_items(review_items: &ReviewCollection,
) -> Result<(), Box<dyn Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Marble Devices", "Range Trainer") {
        fs::create_dir_all(proj_dirs.data_dir())?;
        let data_dir = proj_dirs.data_dir().join("review_items.json");
        let path = data_dir.as_path();
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;

        file.write_all(&serde_json::to_string(review_items)?.as_bytes())?;
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to create app directory.",
        )))
    }
}

pub fn load_ranges() -> Vec<range::Range> {
    if let Some(proj_dirs) = ProjectDirs::from(TLD, ORG, APP) {
        let data_dir = proj_dirs.data_dir().join("ranges.json");
        let path = data_dir.as_path();
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(data) = serde_json::from_str(&contents) {
                return data;
            } else {
                dbg!("Couldn't parse json");
            }
        } else {
            dbg!("Couldnt read contents");
        }
    } else {
        dbg!("couldnt get project dir");
    }
    return vec![];
}

pub fn save_ranges(ranges: &Vec<range::Range>) -> Result<(), Box<dyn Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Marble Devices", "Range Trainer") {
        fs::create_dir_all(proj_dirs.data_dir())?;
        let data_dir = proj_dirs.data_dir().join("ranges.json");
        let path = data_dir.as_path();
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;

        file.write_all(&serde_json::to_string(ranges)?.as_bytes())?;
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to create app directory.",
        )))
    }
}

#[cfg(test)]
mod tests {
    use crate::review;
    use super::ReviewCollection;
    use std::collections::HashMap;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;
    use chrono::prelude::*;
    use rand::Rng;

    impl Arbitrary for review::item::DisplayString {
        fn arbitrary<G: Gen>(g: &mut G) -> review::item::DisplayString {
            review::item::DisplayString::new(&String::arbitrary(g))
        }
    }

    impl Arbitrary for review::item::Presentation {
        fn arbitrary<G: Gen>(g: &mut G) -> review::item::Presentation {
            let x = bool::arbitrary(g);
            if x {
                review::item::Presentation::Text(review::item::DisplayString::arbitrary(g))
            } else {
                review::item::Presentation::TextHand(review::item::DisplayString::arbitrary(g), g.gen())
            }
        }
    }

    impl Arbitrary for review::item::Answer {
        fn arbitrary<G: Gen>(g: &mut G) -> review::item::Answer {
            let x = g.next_u32() % 4;
            match x {
                0 => review::item::Answer::Yes,
                1 => review::item::Answer::No,
                2 => review::item::Answer::Text(review::item::DisplayString::arbitrary(g)),
                3 => review::item::Answer::PokerAction(review::item::PokerAction::arbitrary(g)),
                _ => unreachable!()
            }
        }
    }

    impl Arbitrary for review::item::PokerAction {
        fn arbitrary<G: Gen>(g: &mut G) -> review::item::PokerAction {
           let x = g.next_u32() % 4;
           match x {
               0 => review::item::PokerAction::Fold,
               1 => review::item::PokerAction::Raise(f32::arbitrary(g).to_string()),
               2 => review::item::PokerAction::Check,
               3 => review::item::PokerAction::Call,
               _ => unreachable!()
           }
        }
    }

    #[derive(Clone)]
    pub struct MyDateTimeLocal(DateTime<Local>);
    impl Arbitrary for MyDateTimeLocal {
        fn arbitrary<G: Gen>(g: &mut G) -> MyDateTimeLocal {
            MyDateTimeLocal(Local.timestamp(i64::arbitrary(g), u32::arbitrary(g)))
        }
    }

    impl Arbitrary for review::item::Options {
        fn arbitrary<G: Gen>(g: &mut G) -> review::item::Options {
            let x = g.next_u32() % 3;
            match x {
                0 => review::item::Options::Binary,
                1 => review::item::Options::Numbers,
                2 => review::item::Options::PokerAction,
                _ => unreachable!()
            }
        }
    }

    impl Arbitrary for ReviewCollection {
        fn arbitrary<G: Gen>(g: &mut G) -> ReviewCollection {
            let size = g.next_u32() % 50;
            let mut review_collection = ReviewCollection(HashMap::<(review::item::Presentation, review::item::Answer), review::item::ReviewItem>::new());
            for _ in  0..size {
                let presentation = review::item::Presentation::arbitrary(g);
                let answer = review::item::Answer::arbitrary(g);
                review_collection.0.insert((presentation.clone(), answer.clone()), review::item::ReviewItem {
                    difficulty: (g.next_u32() % 100) as f32 / 100.0,
                    days_between_review_attempts: f32::arbitrary(g),
                    date_last_reviewed: Option::<MyDateTimeLocal>::arbitrary(g).map(|x| x.0),
                    last_answer_correct: bool::arbitrary(g),
                    question: review::item::Question {
                        presentation: presentation,
                        options: review::item::Options::arbitrary(g),
                        answer: answer,
                        tolerance: Option::<f32>::arbitrary(g),
                    }
                });
            }
            review_collection
        }
    }

    #[quickcheck]
    fn deserialized_serialized_reivew_collection_key_is_unchanged(key: (review::item::Presentation, review::item::Answer)) -> bool {
        super::parse_review_collection_key(&super::serialize_review_collection_key(&key)).unwrap() == key
    }

    #[quickcheck]
    fn deserialized_serialized_review_collection_is_unchanged(review_collection: ReviewCollection) -> bool {
        let serialized = serde_json::to_string(&review_collection).unwrap();
        let deserialized: ReviewCollection = serde_json::from_str(&serialized).unwrap();
        deserialized == review_collection
    }
}
