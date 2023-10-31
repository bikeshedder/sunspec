use itertools::Itertools;

pub struct Documentation {
    pub label: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
}

impl ToString for Documentation {
    fn to_string(&self) -> String {
        let parts = [
            ("", &self.label),
            ("", &self.description),
            ("Notes: ", &self.notes),
        ];
        parts
            .into_iter()
            .filter_map(|(label, text)| {
                text.as_ref().and_then(|text| {
                    if text.is_empty() {
                        None
                    } else {
                        Some((label, text))
                    }
                })
            })
            .map(|(label, text)| format!("{}{}", label, text))
            .join("\n\n")
    }
}
