use colored::Colorize;

pub fn parse(words: &[String]) {
    let mut lexicon = crate::domain::Lexicon::load();

    match words {
        [verb, terms @ ..] if verb == "add" && !terms.is_empty() => {
            lexicon.expand(terms);
            println!("{}", "Keywords added.".green());
        }
        [verb, terms @ ..] if verb == "delete" && !terms.is_empty() => {
            lexicon.shrink(terms);
            println!("{}", "Keywords deleted.".green());
        }
        [verb] if verb == "clear" => {
            lexicon.clear();
            println!("{}", "Lexicon cleared.".green());
        }
        [verb, flags @ ..] if verb == "list" => {
            let count = flags.contains(&String::from("--count"));

            let mut order = crate::domain::lexicon::Order::Ascending;

            if let Some(parameter) = flags.iter().find(|flag| flag.starts_with("--sort=")) {
                let value = parameter.trim_start_matches("--sort=");
                order = match value {
                    "desc" => crate::domain::lexicon::Order::Descending,
                    "length" => crate::domain::lexicon::Order::Length,
                    "asc" | _ => crate::domain::lexicon::Order::Ascending,
                };
            }

            lexicon.show(count, order);
        }
        _ => {
            println!(
                "Invalid keywords command. Try {}, {}, {}, or {}.",
                "add".cyan(),
                "list".cyan(),
                "delete".cyan(),
                "clear".cyan()
            );
        }
    }
}
