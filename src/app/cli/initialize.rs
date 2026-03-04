use colored::Colorize;

pub fn parse(words: &[String]) {
    match words {
        [] => {
            println!("{}", "Initializing environment...".green().bold());

            let settings = crate::domain::Settings::default();
            settings.save();
            println!("  {} Created settings.json", "-".cyan());

            let lexicon = crate::domain::Lexicon::default();
            lexicon.save();
            println!("  {} Created lexicon.json", "-".cyan());

            let fleet = crate::domain::Fleet::default();
            fleet.save();
            println!("  {} Created fleet.json", "-".cyan());

            crate::domain::Catalog::seed();
            println!(
                "  {} Created providers.json (seeded with google, protonmail)",
                "-".cyan()
            );

            println!(
                "\n{}",
                "Initialization complete. The application is ready.".green()
            );
        }
        _ => {
            println!("Invalid command. Simply run: {} init", "mailina".cyan());
        }
    }
}
