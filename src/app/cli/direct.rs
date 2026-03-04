use colored::Colorize;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn parse(words: &[String]) {
    let mut settings = crate::domain::Settings::load();

    match words {
        [kind, flag1, val1, flag2, val2]
            if kind == "telegram" && flag1 == "--token" && flag2 == "--chat" =>
        {
            let identifier = generate();
            let destination = crate::domain::Destination::Telegram {
                identifier: identifier.clone(),
                token: val1.clone(),
                chat: val2.clone(),
            };
            settings.attach(destination);
            println!(
                "{} attached. Identifier: {}",
                "Telegram route".green(),
                identifier.cyan()
            );
        }
        [kind, flag, address] if kind == "forward" && flag == "--to" => {
            let identifier = generate();
            let destination = crate::domain::Destination::Forward {
                identifier: identifier.clone(),
                address: address.clone(),
            };
            settings.attach(destination);
            println!(
                "{} attached. Identifier: {}",
                "Forward route".green(),
                identifier.cyan()
            );
        }
        [kind, flag] if kind == "console" && flag == "--enable" => {
            settings.toggle(true);
            println!("{}", "Console output enabled.".green());
        }
        [kind, flag] if kind == "console" && flag == "--disable" => {
            settings.toggle(false);
            println!("{}", "Console output disabled.".green());
        }
        [verb] if verb == "list" => {
            settings.inventory();
        }
        [verb, identifier] if verb == "delete" => {
            settings.detach(identifier);
            println!("{}", "Route detached (if it existed).".green());
        }
        _ => {
            println!(
                "Invalid routing command. Check syntax for {}, {}, {}, {}, or {}.",
                "telegram".cyan(),
                "forward".cyan(),
                "console".cyan(),
                "list".cyan(),
                "delete".cyan()
            );
        }
    }
}

fn generate() -> String {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("rt-{}", stamp)
}
