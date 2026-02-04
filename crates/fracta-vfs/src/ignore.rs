//! Gitignore-style ignore rules for scope determination.
//!
//! Fracta uses `.fracta/config/ignore` to determine which paths within a managed
//! Location should be treated as Ignored (visible but not indexed/processed).
//!
//! Syntax follows `.gitignore` conventions:
//! - `#` comments, blank lines skipped
//! - `!` prefix negates a rule
//! - Trailing `/` matches directories only
//! - `*` and `**` wildcards
//! - Patterns without `/` match anywhere in the tree

use std::path::{Path, PathBuf};

use globset::{Glob, GlobMatcher};

/// A compiled set of ignore rules.
#[derive(Debug, Clone)]
pub struct IgnoreRules {
    rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
struct Rule {
    matcher: GlobMatcher,
    negated: bool,
    dir_only: bool,
}

/// Default ignore patterns applied to every managed Location.
pub const DEFAULT_IGNORE: &str = "\
# Fracta default ignore rules
# Syntax follows .gitignore conventions

# Version control
.git/

# macOS system files
.DS_Store
.Spotlight-V100/
.Trashes/
.fseventsd/
._*

# Common build artifacts
node_modules/
target/
build/
dist/
.cache/

# IDE and editor
.idea/
.vscode/
*.swp
*.swo
*~
";

impl IgnoreRules {
    /// Create an empty ruleset (nothing is ignored).
    pub fn empty() -> Self {
        Self { rules: Vec::new() }
    }

    /// Load rules from a file path. Returns empty rules if the file does not exist.
    pub fn load(path: &Path) -> std::io::Result<Self> {
        if !path.exists() {
            return Ok(Self::empty());
        }
        let content = std::fs::read_to_string(path)?;
        Ok(Self::parse(&content))
    }

    /// Parse rules from a string in gitignore syntax.
    pub fn parse(content: &str) -> Self {
        let rules = content
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    return None;
                }
                Self::compile_rule(trimmed)
            })
            .collect();
        Self { rules }
    }

    fn compile_rule(line: &str) -> Option<Rule> {
        let mut pattern = line;

        // Check for negation prefix
        let negated = pattern.starts_with('!');
        if negated {
            pattern = &pattern[1..];
        }

        // Check for directory-only suffix
        let dir_only = pattern.ends_with('/');
        if dir_only {
            pattern = &pattern[..pattern.len() - 1];
        }

        // Determine anchoring:
        // - Leading `/` → anchored to root (don't prepend **/)
        // - Contains `/` in the middle → anchored to root
        // - Neither → matches anywhere in the tree (prepend **/)
        let anchored = pattern.starts_with('/');
        let stripped = pattern.strip_prefix('/').unwrap_or(pattern);
        let glob_pattern = if anchored || stripped.contains('/') {
            stripped.to_string()
        } else {
            format!("**/{stripped}")
        };

        let glob = Glob::new(&glob_pattern).ok()?;

        Some(Rule {
            matcher: glob.compile_matcher(),
            negated,
            dir_only,
        })
    }

    /// Check whether a relative path is ignored.
    ///
    /// `rel_path` should be relative to the Location root.
    /// `is_dir` indicates whether the path refers to a directory.
    ///
    /// A path is ignored if it matches an ignore rule, OR if any of its ancestor
    /// directories match a directory-ignore rule. This mirrors gitignore behavior
    /// where ignoring a directory implicitly ignores all its contents.
    pub fn is_ignored(&self, rel_path: &Path, is_dir: bool) -> bool {
        // Check each prefix of the path from root to leaf.
        let mut accumulated = PathBuf::new();
        let components: Vec<_> = rel_path.components().collect();

        for (i, component) in components.iter().enumerate() {
            accumulated.push(component);
            let is_last = i == components.len() - 1;
            let check_is_dir = if is_last { is_dir } else { true };

            if self.matches_rules(&accumulated, check_is_dir) {
                return true;
            }
        }

        false
    }

    /// Evaluate rules against a single path segment (no ancestor checking).
    fn matches_rules(&self, rel_path: &Path, is_dir: bool) -> bool {
        let mut ignored = false;
        for rule in &self.rules {
            if rule.dir_only && !is_dir {
                continue;
            }
            if rule.matcher.is_match(rel_path) {
                ignored = !rule.negated;
            }
        }
        ignored
    }
}

impl Default for IgnoreRules {
    fn default() -> Self {
        Self::parse(DEFAULT_IGNORE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_rules_ignore_nothing() {
        let rules = IgnoreRules::empty();
        assert!(!rules.is_ignored(Path::new("anything.txt"), false));
        assert!(!rules.is_ignored(Path::new("any/path"), true));
    }

    #[test]
    fn test_simple_file_pattern() {
        let rules = IgnoreRules::parse("*.log");
        assert!(rules.is_ignored(Path::new("debug.log"), false));
        assert!(rules.is_ignored(Path::new("sub/dir/app.log"), false));
        assert!(!rules.is_ignored(Path::new("readme.md"), false));
    }

    #[test]
    fn test_directory_only_pattern() {
        let rules = IgnoreRules::parse("build/");
        // Directory named "build" is ignored
        assert!(rules.is_ignored(Path::new("build"), true));
        // File named "build" is NOT ignored (dir_only rule)
        assert!(!rules.is_ignored(Path::new("build"), false));
        // Files inside the ignored directory are ignored (ancestor check)
        assert!(rules.is_ignored(Path::new("build/output.js"), false));
    }

    #[test]
    fn test_negation() {
        let rules = IgnoreRules::parse("*.log\n!important.log");
        assert!(rules.is_ignored(Path::new("debug.log"), false));
        assert!(!rules.is_ignored(Path::new("important.log"), false));
    }

    #[test]
    fn test_anchored_pattern() {
        let rules = IgnoreRules::parse("/root_only");
        // At root: ignored
        assert!(rules.is_ignored(Path::new("root_only"), false));
        // Nested: NOT ignored (pattern is anchored)
        assert!(!rules.is_ignored(Path::new("sub/root_only"), false));
    }

    #[test]
    fn test_nested_path_pattern() {
        let rules = IgnoreRules::parse("logs/*.log");
        assert!(rules.is_ignored(Path::new("logs/app.log"), false));
        assert!(!rules.is_ignored(Path::new("other/app.log"), false));
    }

    #[test]
    fn test_ancestor_directory_ignored() {
        let rules = IgnoreRules::parse("node_modules/");
        assert!(rules.is_ignored(Path::new("node_modules"), true));
        assert!(rules.is_ignored(Path::new("node_modules/pkg/index.js"), false));
        assert!(rules.is_ignored(Path::new("node_modules/pkg/lib"), true));
    }

    #[test]
    fn test_default_rules() {
        let rules = IgnoreRules::default();
        assert!(rules.is_ignored(Path::new(".git"), true));
        assert!(rules.is_ignored(Path::new(".DS_Store"), false));
        assert!(rules.is_ignored(Path::new("node_modules"), true));
        assert!(rules.is_ignored(Path::new("project/node_modules"), true));
        assert!(!rules.is_ignored(Path::new("readme.md"), false));
    }

    #[test]
    fn test_comments_and_blank_lines() {
        let rules = IgnoreRules::parse("# comment\n\n*.tmp\n  # indented comment\n");
        assert!(rules.is_ignored(Path::new("test.tmp"), false));
        assert!(!rules.is_ignored(Path::new("test.txt"), false));
    }

    #[test]
    fn test_dotfile_pattern() {
        let rules = IgnoreRules::parse("._*");
        assert!(rules.is_ignored(Path::new("._resource"), false));
        assert!(!rules.is_ignored(Path::new(".hidden"), false));
    }

    #[test]
    fn test_load_nonexistent_file() {
        let rules = IgnoreRules::load(Path::new("/nonexistent/path")).unwrap();
        assert!(!rules.is_ignored(Path::new("anything"), false));
    }
}
