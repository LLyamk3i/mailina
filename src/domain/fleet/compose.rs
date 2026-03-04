use crate::domain::Credential;

impl super::Fleet {
    pub fn enroll(&mut self, unit: Credential) {
        self.units.retain(|existing| existing.email != unit.email);
        self.units.push(unit);
        self.save();
    }

    pub fn discharge(&mut self, emails: &[String]) {
        self.units.retain(|unit| !emails.contains(&unit.email));
        self.save();
    }

    pub fn inventory(&self) {
        if self.units.is_empty() {
            println!("Fleet is empty.");
            return;
        }
        println!("Active Units ({}):", self.units.len());
        for unit in &self.units {
            println!("- {} | {} | {}", unit.email, unit.mask(), unit.domain);
        }
    }

    pub fn absorb(&mut self, payload: &str) {
        let mut count = 0;

        for line in payload.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                let email = parts[0].trim().to_string();
                let password = parts[1].trim().to_string();
                let domain = if parts.len() >= 3 {
                    parts[2].trim().to_string()
                } else {
                    String::from("auto")
                };

                let unit = Credential {
                    email,
                    password,
                    domain,
                };

                self.units.retain(|existing| existing.email != unit.email);
                self.units.push(unit);
                count += 1;
            }
        }

        self.save();
        println!("Absorbed {} units into the fleet.", count);
    }
}
