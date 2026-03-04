use async_native_tls::TlsConnector;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

#[derive(Serialize, Deserialize, Clone)]
pub struct Credential {
    pub email: String,
    pub password: String,
    pub domain: String,
}

impl Credential {
    pub fn mask(&self) -> String {
        let length = self.password.len();
        if length <= 2 {
            return String::from("********");
        }
        let first = self.password.chars().next().unwrap();
        let last = self.password.chars().last().unwrap();
        format!("{}******{}", first, last)
    }

    pub async fn authenticate(&self) -> Result<crate::domain::Mailbox, String> {
        let catalog = crate::domain::Catalog::load();
        let profile = catalog
            .find(&self.domain)
            .ok_or_else(|| String::from("Provider profile not found in catalog"))?;

        let address = format!("{}:{}", profile.host, profile.port);

        let stream = TcpStream::connect(&address)
            .await
            .map_err(|error| format!("TCP connection failed: {}", error))?;

        let connector = TlsConnector::new();
        let secure = connector
            .connect(&profile.host, stream.compat())
            .await
            .map_err(|error| format!("TLS upgrade failed: {}", error))?;

        let client = async_imap::Client::new(secure);

        let connection = client
            .login(&self.email, &self.password)
            .await
            .map_err(|error| format!("Authentication rejected: {}", error.0))?;

        Ok(crate::domain::Mailbox {
            identity: self.email.clone(),
            connection,
        })
    }
}
