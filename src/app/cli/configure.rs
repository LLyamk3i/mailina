use colored::Colorize;

pub fn parse(words: &[String]) {
    let mut settings = crate::domain::Settings::load();

    match words {
        [verb, value] if verb == "interval" => {
            let tick = value.parse::<u64>().expect("invalid number");
            settings.interval = tick;
            settings.save();
            println!("{} set to {}s", "Interval".green(), tick.to_string().cyan());
        }
        [verb, value] if verb == "limit" => {
            let maximum = value.parse::<u32>().expect("invalid number");
            settings.limit = maximum;
            settings.save();
            println!("{} set to {}", "Limit".green(), maximum.to_string().cyan());
        }
        [] => {
            settings.show();
        }
        _ => {
            println!(
                "Invalid configuration command. Use {} or {}.",
                "interval <number>".cyan(),
                "limit <number>".cyan()
            );
        }
    }
}
