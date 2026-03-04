pub async fn post(endpoint: &str, payload: &serde_json::Value) -> Result<(), String> {
    let client = reqwest::Client::new();

    let response = client
        .post(endpoint)
        .json(payload)
        .send()
        .await
        .map_err(|error| format!("Network transmission failed: {}", error))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!(
            "Server rejected payload with status: {}",
            response.status()
        ))
    }
}
