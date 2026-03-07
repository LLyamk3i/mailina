use crate::domain::Credential;

use colored::Colorize;
use tokio::task;

impl super::Fleet {
    // Affordance: The fleet concurrently tests all units and amputates dead credentials
    pub async fn prune(&mut self) {
        if self.units.is_empty() {
            println!("  [INFO] Fleet is empty. Nothing to prune.");
            return;
        }

        let mut tasks = Vec::new();
        println!(
            "  {} Pinging {} units simultaneously...",
            "[NETWORK]".blue().bold(),
            self.units.len()
        );

        // 1. The Concurrent Network Test
        for unit in self.units.clone() {
            let handle = task::spawn(async move {
                // We only care if the physical connection succeeds or fails
                let is_valid = unit.authenticate().await.is_ok();
                (unit.email, is_valid)
            });
            tasks.push(handle);
        }

        // Wait for all TCP handshakes to resolve
        let results = futures::future::join_all(tasks).await;

        // 2. The Harvest
        let mut dead_emails = Vec::new();
        for result in results {
            if let Ok((email, is_valid)) = result {
                if !is_valid {
                    dead_emails.push(email);
                }
            }
        }

        // 3. The Surgical Amputation
        if dead_emails.is_empty() {
            println!(
                "  {} All fleet credentials are authenticated and healthy.",
                "[SUCCESS]".green().bold()
            );
            return;
        }

        let original_count = self.units.len();

        // Retain only the units whose emails are NOT in the dead list
        self.units.retain(|u| !dead_emails.contains(&u.email));
        self.save(); // Persist the cleaned state to disk

        let removed = original_count - self.units.len();
        println!(
            "  {} Fleet pruned. Removed {} dead credentials:",
            "[ALERT]".red().bold(),
            removed
        );

        for email in dead_emails {
            println!("  - Purged: {}", email.dimmed());
        }
    }

    pub fn enroll(&mut self, unit: Credential) {
        self.units.retain(|existing| existing.email != unit.email);
        self.units.push(unit);
        self.save();
    }

    pub fn discharge(&mut self, emails: &[String]) {
        self.units.retain(|unit| !emails.contains(&unit.email));
        self.save();
    }

    pub fn inventory(&self) {
        if self.units.is_empty() {
            println!("Fleet is empty.");
            return;
        }
        println!("Active Units ({}):", self.units.len());
        for unit in &self.units {
            println!("- {} | {} | {}", unit.email, unit.mask(), unit.domain);
        }
    }

    pub fn absorb(&mut self, payload: &str) {
        let mut count = 0;

        for line in payload.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                let email = parts[0].trim().to_string();
                let password = parts[1].trim().to_string();
                let domain = if parts.len() >= 3 {
                    parts[2].trim().to_string()
                } else {
                    String::from("auto")
                };

                let unit = Credential {
                    email,
                    password,
                    domain,
                };

                self.units.retain(|existing| existing.email != unit.email);
                self.units.push(unit);
                count += 1;
            }
        }

        self.save();
        println!("Absorbed {} units into the fleet.", count);
    }
}
