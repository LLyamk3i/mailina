use colored::Colorize;
use tokio::task;

impl super::Fleet {
    pub async fn survey(
        &self,
        targets: &[String],
        limit: u32,
        lexicon: &crate::domain::Lexicon,
        destinations: &[crate::domain::Destination],
        filter: bool,
    ) {
        let mut tasks = Vec::new();

        let active_units: Vec<_> = if targets.is_empty() {
            self.units.clone()
        } else {
            self.units
                .iter()
                .filter(|u| targets.contains(&u.email))
                .cloned()
                .collect()
        };

        if active_units.is_empty() {
            println!("  [INFO] No fleet units matched the target list.");
            return;
        }

        for unit in active_units {
            let local_lexicon = lexicon.words.clone();
            let local_destinations = destinations.to_vec();

            let handle = task::spawn(async move {
                let mut mailbox = match unit.authenticate().await {
                    Ok(session) => session,
                    Err(error) => {
                        let badge = "[ERROR]".red().bold();
                        println!(
                            "  {} Authentication failed for {}: {}",
                            badge,
                            unit.email.yellow(),
                            error
                        );
                        return;
                    }
                };

                let messages = match mailbox.read(limit).await {
                    Ok(found) => found,
                    Err(error) => {
                        let badge = "[ERROR]".red().bold();
                        println!(
                            "  {} Mailbox extraction failed for {}: {}",
                            badge,
                            unit.email.yellow(),
                            error
                        );
                        return;
                    }
                };

                let badge = "[INFO]".cyan().bold();
                println!(
                    "  {} Read {} messages from {}",
                    badge,
                    messages.len().to_string().yellow(),
                    unit.email.cyan()
                );

                let thread_lexicon = crate::domain::Lexicon {
                    words: local_lexicon,
                };

                // We declare the message as mutable so it can absorb its tags
                for mut message in messages {
                    let matched = if filter {
                        // The message mutates its own state during evaluation
                        message.evaluate(&thread_lexicon)
                    } else {
                        true
                    };

                    if !matched {
                        continue;
                    }

                    if filter {
                        let badge = "[MATCH]".green().bold();
                        println!("  {} Keyword found! Routing message...", badge);
                    } else {
                        let badge = "[FETCH]".blue().bold();
                        println!(
                            "  {} Message retrieved (Filtering disabled). Routing...",
                            badge
                        );
                    }

                    for target in &local_destinations {
                        target.receive(&message).await;
                    }
                }
            });
            tasks.push(handle);
        }

        futures::future::join_all(tasks).await;
    }
}
