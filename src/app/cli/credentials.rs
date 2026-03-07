use colored::Colorize;
use std::io::{self, Read};

pub async fn parse(words: &[String]) {
    let mut fleet = crate::domain::Fleet::load();

    match words {
        [verb] if verb == "clean" => {
            println!("{}", "Commencing fleet health check...".cyan().bold());
            fleet.prune().await;
        }
        [verb, flags @ ..] if verb == "add" => {
            let mut email = String::new();
            let mut password = String::new();
            let mut domain = String::from("auto");

            let mut iterator = flags.iter();
            while let Some(flag) = iterator.next() {
                match flag.as_str() {
                    "--email" => {
                        if let Some(value) = iterator.next() {
                            email = value.clone();
                        }
                    }
                    "--password" => {
                        if let Some(value) = iterator.next() {
                            password = value.clone();
                        }
                    }
                    "--domain" => {
                        if let Some(value) = iterator.next() {
                            domain = value.clone();
                        }
                    }
                    _ => {}
                }
            }

            if email.is_empty() || password.is_empty() {
                println!(
                    "{}: Missing required flags {} and {}",
                    "Error".red().bold(),
                    "--email".cyan(),
                    "--password".cyan()
                );
                return;
            }

            let unit = crate::domain::Credential {
                email,
                password,
                domain,
            };
            fleet.enroll(unit);
            println!("{}", "Unit enrolled successfully.".green());
        }
        [verb, path] if verb == "import" => {
            let payload = std::fs::read_to_string(path).unwrap_or_default();
            if payload.is_empty() {
                println!("{}", "File is empty or could not be read.".red());
                return;
            }
            fleet.absorb(&payload);
        }
        [verb] if verb == "import" => {
            let mut payload = String::new();
            io::stdin().read_to_string(&mut payload).unwrap_or_default();
            if payload.is_empty() {
                println!("{}", "No input provided via standard input.".red());
                return;
            }
            fleet.absorb(&payload);
        }
        [verb] if verb == "list" => {
            fleet.inventory();
        }
        [verb, targets @ ..] if verb == "delete" && !targets.is_empty() => {
            fleet.discharge(targets);
            println!("{}", "Units discharged successfully.".green());
        }
        _ => {
            println!(
                "Invalid credentials command. Try {}, {}, {}, {}, or {}.",
                "add".cyan(),
                "import".cyan(),
                "list".cyan(),
                "delete".cyan(),
                "clean".cyan()
            );
        }
    }
}
