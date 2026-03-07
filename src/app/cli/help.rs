use colored::Colorize;

pub fn show(words: &[String]) {
    // Safely extract the first physical word that isn't a flag, regardless of order
    let target = words
        .iter()
        .find(|w| !w.starts_with('-'))
        .map(|s| s.as_str());

    match target {
        Some("config") => {
            println!("{} - Manage application settings", "config".cyan().bold());
            println!("\nUsage:");
            println!("  mailina config interval <seconds>  Set polling frequency");
            println!("  mailina config limit <number>      Set max messages per fetch");
            println!("  mailina config                     Show current settings");
        }
        Some("credentials") => {
            println!(
                "{} - Manage account credentials",
                "credentials".cyan().bold()
            );
            println!("\nUsage:");
            println!("  mailina credentials add --email <e> --password <p> [--domain <d>]");
            println!("  mailina credentials import <path>    Import colon-separated file");
            println!("  mailina credentials list             Show all active units");
            println!("  mailina credentials delete <email>  Remove a unit from the fleet");
            println!("  mailina credentials clean           Test and remove dead credentials");
        }
        Some("fetch") => {
            println!("{} - Perform one-time email survey", "fetch".cyan().bold());
            println!("\nUsage:");
            println!("  mailina {} [--filter] [--only <emails>]", "fetch".cyan());
            println!("\nFlags:");
            println!("  --filter               Enable lexicon filtering (OFF by default)");
            println!("  --only <email1,email2> Target specific units instead of the whole fleet");
        }
        Some("run") => {
            println!("{} - Start continuous polling loop", "run".cyan().bold());
            println!("\nUsage:");
            println!("  mailina {} [--no-filter] [--only <emails>]", "run".cyan());
            println!("\nFlags:");
            println!("  --no-filter            Disable lexicon filtering (ON by default)");
            println!("  --only <email1,email2> Target specific units instead of the whole fleet");
        }
        Some("audit") => {
            println!("{} - Perform deep historical scan", "audit".cyan().bold());
            println!("\nUsage:");
            println!("  mailina {} [--only <emails>]", "audit".cyan());
            println!("\nFlags:");
            println!("  --only <email1,email2> Target specific units instead of the whole fleet");
        }
        Some("keywords") => {
            println!("{} - Manage keyword lexicon", "keywords".cyan().bold());
            println!("\nUsage:");
            println!("  mailina keywords add <words...>     Add words to lexicon");
            println!("  mailina keywords delete <words...>  Remove words from lexicon");
            println!("  mailina keywords list [--count]     Show all keywords");
            println!("  mailina keywords clear              Delete all keywords");
        }
        Some("routing") => {
            println!("{} - Manage alert destinations", "routing".cyan().bold());
            println!("\nUsage:");
            println!("  mailina routing telegram --token <t> --chat <c>");
            println!("  mailina routing console --enable|--disable");
            println!("  mailina routing list");
            println!("  mailina routing delete <id>");
        }
        _ => {
            println!(
                "{}",
                "Mailina - CLI Email Polling and Alerting Engine"
                    .green()
                    .bold()
            );
            println!("\nCommands:");
            println!("  init         Initialize storage and defaults");
            println!("  config       Manage settings (interval, limit)");
            println!("  routing      Manage alert destinations");
            println!("  keywords     Manage filtering keywords");
            println!("  credentials  Manage email accounts");
            println!("  fetch        Perform immediate fetch (All messages)");
            println!("  run          Start continuous polling (Filtered messages)");
            println!("  audit        Deep historical scan with pagination");
            println!("\nGlobal Flags:");
            println!("  --help, -h    Show contextual help");
            println!("  --version, -V Show application version");
        }
    }
}
