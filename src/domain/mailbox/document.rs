use mailparse::ParsedMail;

// Stateless, recursive helper strictly scoped to extracting text from MIME trees.
// We expose it to the sibling files in this directory via `pub(super)`.
pub(super) fn extract(document: &ParsedMail) -> String {
    let mimetype = document.ctype.mimetype.clone();

    // Base Case
    if mimetype == "text/plain" {
        return document.get_body().unwrap_or_default().trim().to_string();
    }

    let mut content = String::new();

    // Recursive Case: Dig into multipart segments
    for segment in &document.subparts {
        let text = extract(segment);
        if !text.is_empty() {
            if segment.ctype.mimetype == "text/plain" {
                return text;
            }
            content.push_str(&text);
            content.push('\n');
        }
    }

    // Fallback for HTML-only emails
    if document.subparts.is_empty() && mimetype == "text/html" {
        return document.get_body().unwrap_or_default().trim().to_string();
    }

    content.trim().to_string()
}
