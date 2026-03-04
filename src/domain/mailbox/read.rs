use colored::Colorize;
use futures::StreamExt;

// We extend the parent Mailbox struct natively
impl super::Mailbox {
    pub async fn read(&mut self, limit: u32) -> Result<Vec<crate::domain::Message>, String> {
        println!(
            "  {} Mailbox [{}] surveying inbox...",
            "->".blue(),
            self.identity.cyan()
        );

        let inbox = self
            .connection
            .select("INBOX")
            .await
            .map_err(|error| format!("Failed to select INBOX: {}", error))?;

        let total = inbox.exists;
        println!(
            "  {} Mailbox [{}] has {} messages.",
            "->".blue(),
            self.identity.cyan(),
            total.to_string().yellow()
        );

        if total == 0 {
            return Ok(Vec::new());
        }

        let start = if total > limit { total - limit + 1 } else { 1 };
        let sequence = format!("{}:{}", start, total);
        let query = "(BODY.PEEK[])";

        let mut fetches = self
            .connection
            .fetch(sequence, query)
            .await
            .map_err(|error| format!("Fetch request rejected: {}", error))?;

        let mut collection = Vec::new();

        while let Some(result) = fetches.next().await {
            let fetch = match result {
                Ok(data) => data,
                Err(error) => {
                    let badge = "[ERROR]".red().bold();
                    println!(
                        "  {} Mailbox stream corrupted or rejected: {}",
                        badge, error
                    );
                    continue;
                }
            };

            if let Some(bytes) = fetch.body() {
                if let Ok(document) = mailparse::parse_mail(bytes) {
                    let subject = document
                        .headers
                        .iter()
                        .find(|header| header.get_key().to_lowercase() == "subject")
                        .map(|header| header.get_value())
                        .unwrap_or_default();

                    let mut message = crate::domain::Message {
                        sender: self.identity.clone(),
                        subject,
                        body: String::new(),
                        tags: Vec::new(),
                    };

                    // Using the pure function from document.rs
                    message.body = super::document::extract(&document);
                    collection.push(message);
                }
            }
        }

        Ok(collection)
    }
}
