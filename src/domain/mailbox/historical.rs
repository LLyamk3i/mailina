use colored::Colorize;
use futures::StreamExt;

// We extend the parent Mailbox struct natively
impl super::Mailbox {
    pub async fn total(&mut self) -> Result<u32, String> {
        let inbox = self
            .connection
            .select("INBOX")
            .await
            .map_err(|error| format!("Failed to select INBOX: {}", error))?;
        Ok(inbox.exists)
    }

    pub async fn read_slice(
        &mut self,
        start: u32,
        end: u32,
    ) -> Result<Vec<crate::domain::Message>, String> {
        let sequence = format!("{}:{}", start, end);
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

        collection.reverse(); // Newest first
        Ok(collection)
    }
}
