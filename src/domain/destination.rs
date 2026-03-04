use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Destination {
    Console,
    Telegram {
        identifier: String,
        token: String,
        chat: String,
    },
    Forward {
        identifier: String,
        address: String,
    },
}

impl Destination {
    pub fn identity(&self) -> String {
        match self {
            Destination::Console => String::from("console"),
            Destination::Telegram { identifier, .. } => identifier.clone(),
            Destination::Forward { identifier, .. } => identifier.clone(),
        }
    }

    pub fn show(&self) {
        match self {
            Destination::Console => println!("[console] Standard Output"),
            Destination::Telegram {
                identifier, chat, ..
            } => {
                println!("[{}] Telegram (Chat: {})", identifier, chat);
            }
            Destination::Forward {
                identifier,
                address,
            } => {
                println!("[{}] Forward (Address: {})", identifier, address);
            }
        }
    }

    pub async fn receive(&self, message: &crate::domain::Message) {
        match self {
            Destination::Console => {
                let divider = "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".dimmed();
                let badge = "[ALERT]".red().bold();
                let sender = message.sender.cyan();
                let subject = message.subject.yellow();

                let snippet: String = if message.body.chars().count() > 100 {
                    let truncated: String = message.body.chars().take(100).collect();
                    format!("{}...", truncated)
                } else {
                    message.body.clone()
                };

                // Formatter for the tags
                let tags_display = if message.tags.is_empty() {
                    String::from("Unfiltered").dimmed()
                } else {
                    message.tags.join(", ").magenta().bold()
                };

                println!("\n  {}", divider);
                println!("  {} Match: {}", badge, sender);
                println!("  Tags:    {}", tags_display);
                println!("  Subject: {}", subject);
                println!("  Snippet: {}", snippet.dimmed());
                println!("  {}\n", divider);
            }
            Destination::Telegram { token, chat, .. } => {
                let text = format!(
                    "📬 Match Found!\nFrom: {}\nSubject: {}\n\n{}",
                    message.sender, message.subject, message.body
                );

                let endpoint = format!("https://api.telegram.org/bot{}/sendMessage", token);

                let payload = serde_json::json!({
                    "chat_id": chat,
                    "text": text,
                });

                match crate::io::net::post(&endpoint, &payload).await {
                    Ok(_) => println!("  [SUCCESS] Telegram alert dispatched."),
                    Err(error) => println!("  [ERROR] Telegram dispatch failed: {}", error),
                }
            }
            Destination::Forward { address, .. } => {
                println!(
                    "  [STUB] Forwarding to {} via SMTP not yet implemented.",
                    address
                );
            }
        }
    }
}
