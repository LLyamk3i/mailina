use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub interval: u64,
    pub limit: u32,
    pub destinations: Vec<crate::domain::Destination>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            interval: 60,
            limit: 1,
            destinations: vec![crate::domain::Destination::Console],
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let raw = crate::io::disk::read("settings.json");
        serde_json::from_str(&raw).unwrap_or_default()
    }

    pub fn save(&self) {
        let raw = serde_json::to_string_pretty(self).expect("serialization failed");
        crate::io::disk::write("settings.json", &raw);
    }

    pub fn attach(&mut self, destination: crate::domain::Destination) {
        self.destinations.push(destination);
        self.save();
    }

    pub fn detach(&mut self, identifier: &str) {
        self.destinations
            .retain(|target| target.identity() != identifier);
        self.save();
    }

    pub fn toggle(&mut self, active: bool) {
        self.detach("console");
        if active {
            self.attach(crate::domain::Destination::Console);
        }
    }

    pub fn inventory(&self) {
        if self.destinations.is_empty() {
            println!("{}", "No active routes.".yellow());
            return;
        }
        for target in &self.destinations {
            target.show();
        }
    }

    pub fn show(&self) {
        println!(
            "{}: {}s",
            "Interval".cyan(),
            self.interval.to_string().yellow()
        );
        println!("{}:    {}", "Limit".cyan(), self.limit.to_string().yellow());
        println!(
            "{}:   {}",
            "Routes".cyan(),
            self.destinations.len().to_string().yellow()
        );
    }
}
