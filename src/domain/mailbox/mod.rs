pub mod document;
pub mod historical;
pub mod read;

use async_imap::Session;
use async_native_tls::TlsStream;
use tokio::net::TcpStream;
use tokio_util::compat::Compat;

pub struct Mailbox {
    pub identity: String,
    // The physical reality of the live network socket
    pub connection: Session<TlsStream<Compat<TcpStream>>>,
}
