use colored::Colorize;
use tokio::time::{Duration, sleep};

// Stateless affordance to safely extract comma-separated targets
fn extract_targets(flags: &[String]) -> Vec<String> {
    if let Some(pos) = flags.iter().position(|arg| arg == "--only") {
        if let Some(emails) = flags.get(pos + 1) {
            return emails.split(',').map(|s| s.trim().to_string()).collect();
        }
    }
    Vec::new()
}

pub async fn parse(words: &[String]) {
    let settings = crate::domain::Settings::load();
    let lexicon = crate::domain::Lexicon::load();
    let fleet = crate::domain::Fleet::load();

    if fleet.units.is_empty() {
        println!("Cannot execute: Fleet is empty. Add credentials first.");
        return;
    }

    match words {
        [verb, flags @ ..] if verb == "fetch" => {
            let filter = flags.contains(&String::from("--filter"));
            let targets = extract_targets(flags);

            println!(
                "Performing immediate fetch across {} units...",
                fleet.units.len().to_string().cyan()
            );
            fleet
                .survey(&targets, settings.limit, &lexicon, &settings.destinations, filter)
                .await;
            println!("{}", "Fetch complete.".green());
        }
        // AUDIT: Deep historical scan with pagination
        [verb, flags @ ..] if verb == "audit" => {
            let targets = extract_targets(flags);

            println!("{}", "Starting historical deep audit.".magenta().bold());
            // We set the chunk size to 100 emails at a time to protect RAM
            fleet.audit(&targets, 100, &lexicon, &settings.destinations).await;
        }

        [verb, flags @ ..] if verb == "run" => {
            let filter = !flags.contains(&String::from("--no-filter"));
            let targets = extract_targets(flags);

            println!("{}", "Starting continuous polling.".green().bold());
            println!(
                "Units: {} | Limit: {} | Interval: {}s | Filter: {}",
                fleet.units.len().to_string().cyan(),
                settings.limit.to_string().cyan(),
                settings.interval.to_string().cyan(),
                filter.to_string().magenta()
            );

            loop {
                fleet
                    .survey(&targets, settings.limit, &lexicon, &settings.destinations, filter)
                    .await;

                println!(
                    "{}",
                    format!("Sleeping for {} seconds...", settings.interval).dimmed()
                );
                sleep(Duration::from_secs(settings.interval)).await;
            }
        }
        _ => {
            println!("Invalid execution command. Try 'run' or 'fetch'.");
        }
    }
}
