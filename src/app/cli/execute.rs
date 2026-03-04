use colored::Colorize;
use tokio::time::{Duration, sleep};

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

            println!(
                "Performing immediate fetch across {} units...",
                fleet.units.len().to_string().cyan()
            );
            fleet
                .survey(settings.limit, &lexicon, &settings.destinations, filter)
                .await;
            println!("{}", "Fetch complete.".green());
        }
        [verb, flags @ ..] if verb == "run" => {
            let filter = !flags.contains(&String::from("--no-filter"));

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
                    .survey(settings.limit, &lexicon, &settings.destinations, filter)
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
