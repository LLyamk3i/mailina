use colored::Colorize;
use tokio::task;

impl super::Fleet {
    pub async fn audit(
        &self,
        targets: &[String],
        chunk_size: u32,
        lexicon: &crate::domain::Lexicon,
        destinations: &[crate::domain::Destination],
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
                        println!("  {} {}", "[ERROR]".red().bold(), error);
                        return;
                    }
                };

                let total = mailbox.total().await.unwrap_or(0);
                if total == 0 {
                    println!("  {} {} is empty.", "[INFO]".cyan().bold(), unit.email);
                    return;
                }

                println!(
                    "  {} {} has {} total emails. Beginning deep scan...",
                    "[AUDIT]".magenta().bold(),
                    unit.email,
                    total
                );

                let mut current_end = total;

                // 1. The Network Loop (Sequential Pagination)
                while current_end > 0 {
                    let start = if current_end > chunk_size {
                        current_end - chunk_size + 1
                    } else {
                        1
                    };

                    println!(
                        "  {} Fetching slice {}:{} from {}",
                        "[NETWORK]".blue(),
                        start,
                        current_end,
                        unit.email
                    );

                    let messages = mailbox
                        .read_slice(start, current_end)
                        .await
                        .unwrap_or_default();
                    let thread_lexicon = crate::domain::Lexicon {
                        words: local_lexicon.clone(),
                    };

                    // 2. The CPU Loop (Concurrent Evaluation)
                    let mut evaluation_tasks = Vec::new();

                    for mut message in messages {
                        let eval_lexicon = thread_lexicon.clone();
                        let eval_destinations = local_destinations.clone();

                        let eval_handle = task::spawn(async move {
                            if message.evaluate(&eval_lexicon) {
                                for target in &eval_destinations {
                                    target.receive(&message).await;
                                }
                            }
                        });
                        evaluation_tasks.push(eval_handle);
                    }

                    futures::future::join_all(evaluation_tasks).await;

                    if start == 1 {
                        break;
                    }
                    current_end = start - 1;
                }

                println!(
                    "  {} Historical audit complete for {}.",
                    "[SUCCESS]".green().bold(),
                    unit.email
                );
            });
            tasks.push(handle);
        }

        futures::future::join_all(tasks).await;
    }
}
