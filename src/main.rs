mod app;
mod domain;
mod io;

use std::env;

#[tokio::main]
async fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();

    crate::app::cli::route(&arguments).await;
}
