//! YAML front matter extraction and parsing.
//!
//! Fracta Markdown files use YAML front matter (delimited by `---`) to store
//! metadata like title, date, tags, and area. The Note engine extracts and
//! parses this, but does not interpret the fields — that's the Framework's job.

use serde_yaml::Value;

/// Parsed YAML front matter from a Markdown file.
#[derive(Debug, Clone, PartialEq)]
pub struct FrontMatter {
    /// The raw YAML string (without `---` delimiters).
    pub raw: String,
    /// Parsed YAML value (dynamic, any shape).
    pub fields: Value,
}

impl FrontMatter {
    /// Parse a front matter string from comrak.
    ///
    /// comrak's `FrontMatter` node includes the delimiter lines (`---`),
    /// so we strip them before parsing.
    pub fn parse(raw_with_delimiters: &str) -> Option<Self> {
        let yaml = strip_delimiters(raw_with_delimiters);
        if yaml.is_empty() {
            return None;
        }

        let fields: Value = serde_yaml::from_str(&yaml).ok()?;

        // Only accept mapping (key-value) front matter, not scalars or sequences
        if !fields.is_mapping() {
            return None;
        }

        Some(Self { raw: yaml, fields })
    }

    /// Get a string field by key.
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.fields.get(key)?.as_str()
    }

    /// Get an integer field by key.
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.fields.get(key)?.as_i64()
    }

    /// Get a float field by key.
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.fields.get(key)?.as_f64()
    }

    /// Get a boolean field by key.
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.fields.get(key)?.as_bool()
    }

    /// Get a sequence of strings by key (e.g., tags).
    pub fn get_string_list(&self, key: &str) -> Option<Vec<&str>> {
        let seq = self.fields.get(key)?.as_sequence()?;
        seq.iter().map(|v| v.as_str()).collect()
    }
}

/// Strip `---` delimiter lines from front matter content.
fn strip_delimiters(raw: &str) -> String {
    raw.lines()
        .filter(|line| line.trim() != "---")
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_front_matter() {
        let input = "---\ntitle: Hello World\ndate: 2025-01-01\n---\n";
        let fm = FrontMatter::parse(input).unwrap();
        assert_eq!(fm.get_str("title"), Some("Hello World"));
        assert_eq!(fm.get_str("date"), Some("2025-01-01"));
    }

    #[test]
    fn test_parse_with_tags() {
        let input = "---\ntitle: Test\ntags: [rust, fracta]\n---\n";
        let fm = FrontMatter::parse(input).unwrap();
        let tags = fm.get_string_list("tags").unwrap();
        assert_eq!(tags, vec!["rust", "fracta"]);
    }

    #[test]
    fn test_parse_numeric_fields() {
        let input = "---\nmood: 7\nscore: 4.56\n---\n";
        let fm = FrontMatter::parse(input).unwrap();
        assert_eq!(fm.get_i64("mood"), Some(7));
        assert_eq!(fm.get_f64("score"), Some(4.56));
    }

    #[test]
    fn test_parse_boolean_field() {
        let input = "---\ndraft: true\n---\n";
        let fm = FrontMatter::parse(input).unwrap();
        assert_eq!(fm.get_bool("draft"), Some(true));
    }

    #[test]
    fn test_empty_front_matter() {
        let input = "---\n---\n";
        assert!(FrontMatter::parse(input).is_none());
    }

    #[test]
    fn test_non_mapping_rejected() {
        // A scalar YAML value should not be accepted as front matter
        let input = "---\njust a string\n---\n";
        assert!(FrontMatter::parse(input).is_none());
    }

    #[test]
    fn test_missing_field_returns_none() {
        let input = "---\ntitle: Test\n---\n";
        let fm = FrontMatter::parse(input).unwrap();
        assert_eq!(fm.get_str("nonexistent"), None);
    }
}
