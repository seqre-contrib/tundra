use std::cell::RefCell;

use anitomy::{Anitomy, ElementCategory};
use log::debug;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Title {
    pub title: String,
    pub season_number: i32,
    pub episode_number: i32,
}

impl Title {
    pub fn new(title: String, season_number: i32, episode_number: i32) -> Self {
        Self {
            title,
            season_number,
            episode_number,
        }
    }
}

#[derive(Default)]
pub struct TitleRecognizer {}

thread_local! {
    static ANITOMY: RefCell<Anitomy> = RefCell::new(Anitomy::new());
}

impl TitleRecognizer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn recognize(&mut self, filename: &str) -> Option<Title> {
        ANITOMY.with(|anitomy| match anitomy.borrow_mut().parse(filename) {
            Ok(ref elements) => {
                debug!("Found path elements: {:?}", elements);
                let title = elements.get(ElementCategory::AnimeTitle)?.to_owned();

                let episode_number: i32 = elements
                    .get(ElementCategory::EpisodeNumber)
                    .unwrap_or("1")
                    .parse()
                    .ok()?;
                if episode_number < 1 {
                    return None;
                }

                let season_number: i32 = elements
                    .get(ElementCategory::AnimeSeason)
                    .unwrap_or("1")
                    .parse()
                    .ok()?;

                Some(Title::new(title, season_number, episode_number))
            }
            Err(_elements) => None,
        })
    }
}
