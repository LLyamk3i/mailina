use colored::Colorize;

pub async fn route(words: &[String]) {
    // 1. THE GLOBAL INTERCEPTORS
    if words.contains(&String::from("--version")) || words.contains(&String::from("-V")) {
        println!("Mailina v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if words.contains(&String::from("--help")) || words.contains(&String::from("-h")) {
        super::help::show(words);
        return;
    }

    // 2. THE LEXICAL ROUTER
    match words {
        [command, subcommands @ ..] if command == "init" => {
            super::initialize::parse(subcommands);
        }
        [command, subcommands @ ..] if command == "config" => {
            super::configure::parse(subcommands);
        }
        [command, subcommands @ ..] if command == "routing" => {
            super::direct::parse(subcommands);
        }
        [command, subcommands @ ..] if command == "keywords" => {
            super::keywords::parse(subcommands);
        }
        [command, subcommands @ ..] if command == "credentials" => {
            super::credentials::parse(subcommands);
        }
        [command, subcommands @ ..]
            if command == "run" || command == "fetch" || command == "audit" =>
        {
            super::execute::parse(words).await;
        }
        [] => println!("No command provided. Try: {} --help", "mailina".cyan()),
        _ => println!("Unknown command. Try: {} --help", "mailina".cyan()),
    }
}
