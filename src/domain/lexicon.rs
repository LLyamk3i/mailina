use colored::Colorize;
use serde::{Deserialize, Serialize};

pub enum Order {
    Ascending,
    Descending,
    Length,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Lexicon {
    pub words: Vec<String>,
}

impl Lexicon {
    pub fn load() -> Self {
        let raw = crate::io::disk::read("lexicon.json");
        // 1. Parse the potentially dirty JSON from the stubborn user
        let parsed: Self = serde_json::from_str(&raw).unwrap_or_default();

        // 2. Enforce the Domain Rule exactly once during boot
        let clean: Vec<String> = parsed
            .words
            .into_iter()
            .map(|word| word.to_lowercase())
            .collect();

        Self { words: clean }
    }

    pub fn save(&self) {
        let raw = serde_json::to_string_pretty(self).expect("serialization failed");
        crate::io::disk::write("lexicon.json", &raw);
    }

    pub fn expand(&mut self, terms: &[String]) {
        for term in terms {
            let lower = term.to_lowercase();
            if !self.words.contains(&lower) {
                self.words.push(lower);
            }
        }
        self.save();
    }

    pub fn shrink(&mut self, terms: &[String]) {
        let targets: Vec<String> = terms.iter().map(|w| w.to_lowercase()).collect();
        self.words.retain(|word| !targets.contains(word));
        self.save();
    }

    pub fn clear(&mut self) {
        self.words.clear();
        self.save();
    }

    pub fn show(&self, count: bool, order: crate::domain::lexicon::Order) {
        if count {
            println!(
                "{}: {}",
                "Total keywords".cyan(),
                self.words.len().to_string().yellow()
            );
        }

        if self.words.is_empty() {
            println!("{}", "Lexicon is empty.".yellow());
            return;
        }

        let mut display = self.words.clone();

        match order {
            crate::domain::lexicon::Order::Ascending => display.sort(),
            crate::domain::lexicon::Order::Descending => {
                display.sort();
                display.reverse();
            }
            crate::domain::lexicon::Order::Length => {
                display.sort_by_key(|word| word.len());
            }
        }

        for word in display {
            println!("  {} {}", "-".cyan(), word);
        }
    }
}
