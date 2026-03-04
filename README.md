# Mailina

Mailina is a lightweight, efficient CLI tool for email polling and alerting, built in Rust. It allows you to monitor multiple email accounts, filter messages based on keywords, and route alerts to various destinations like the console or Telegram.

## Philosophy

Mailina is designed with a strict architectural philosophy:

*   **Affordances over Abilities**: Domain objects own their data and behavior.
*   **Stateless Namespaces**: Logic is grouped in modules, not arbitrary manager classes.
*   **The "One Word" Rule**: Commands and variables use single, descriptive English words.

## Installation

Ensure you have Rust and Cargo installed.

```bash
git clone https://github.com/yourusername/mailina.git
cd mailina
cargo build --release
```

The binary will be located in `target/release/mailina`.

## Usage

### 1. Initialization
Initialize the local storage and configuration.

```bash
./mailina init
```

### 2. Credentials
Manage the fleet of email accounts to monitor.

```bash
# Add a new account
./mailina credentials add --email user@example.com --password "secret" --domain imap.example.com

# Import accounts from a file
./mailina credentials import ./accounts.txt

# List active accounts
./mailina credentials list
```

### 3. Keywords (Lexicon)
Define keywords to filter incoming messages.

```bash
# Add keywords
./mailina keywords add "urgent" "alert" "invoice"

# List keywords
./mailina keywords list

# Remove keywords
./mailina keywords delete "invoice"
```

### 4. Routing
Configure where you want to receive alerts.

```bash
# Enable console output
./mailina routing console --enable

# Add Telegram notification
./mailina routing telegram --token "YOUR_BOT_TOKEN" --chat "CHAT_ID"
```

### 5. Configuration
Tune the polling behavior.

```bash
# Set polling interval to 60 seconds
./mailina config interval 60

# Fetch max 5 messages per cycle
./mailina config limit 5
```

### 6. Execution

**Fetch (One-time):**
Survey inboxes once. By default, this fetches *all* messages. Use `--filter` to apply the keyword lexicon.

```bash
./mailina fetch
./mailina fetch --filter
```

**Run (Continuous):**
Start the polling loop. By default, this *filters* messages using the lexicon. Use `--no-filter` to capture everything.

```bash
./mailina run
./mailina run --no-filter
```

## Project Structure

*   `src/domain`: Domain Nouns (Structs + Affordances).
*   `src/app`: Stateless Namespaces (Logic/Routing).
*   `src/io`: I/O Interfaces.

## License

[License Information Here]
