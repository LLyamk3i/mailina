#[derive(Clone)]
pub struct Message {
    pub sender: String,
    pub subject: String,
    pub body: String,
    // The physical reality of the acquired keywords
    pub tags: Vec<String>,
}

impl Message {
    // Affordance: A message evaluates itself against a lexicon and absorbs the matches
    pub fn evaluate(&mut self, lexicon: &crate::domain::Lexicon) -> bool {
        if lexicon.words.is_empty() {
            return false;
        }

        let haystack = format!("{} {}", self.subject, self.body).to_lowercase();

        // We filter the lexicon for matches and absorb them directly into our state
        self.tags = lexicon
            .words
            .iter()
            .filter(|&word| haystack.contains(word))
            .cloned() // Converts &String back into owned String
            .collect();

        // If we acquired any tags, the evaluation is a success
        !self.tags.is_empty()
    }
}
