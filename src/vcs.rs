use anyhow::Result;
use std::process::Command;

pub fn run_git_command(args: &[&str]) -> Result<String> {
    let output = Command::new("git").args(args).output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git command failed: {}", stderr.trim());
    }
}

pub fn validate_branch_name(branch: &str) -> Result<()> {
    if branch.trim().is_empty() {
        anyhow::bail!("Branch name cannot be empty");
    }
    if branch.starts_with('-') {
        anyhow::bail!("Branch name cannot start with '-'");
    }
    if branch == "@" || branch.contains("@{") {
        anyhow::bail!("Branch name contains invalid ref syntax");
    }

    let output = Command::new("git")
        .args(["check-ref-format", "--branch", branch])
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Invalid branch name '{}': {}", branch, stderr.trim());
    }
}

pub fn git_branch_exists(branch: &str) -> bool {
    if validate_branch_name(branch).is_err() {
        return false;
    }

    let ref_name = format!("refs/heads/{}", branch);
    Command::new("git")
        .args(["show-ref", "--verify", "--quiet", &ref_name])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub fn generate_branch_name(identifier: &str, title: &str) -> String {
    const MAX_SLUG_CHARS: usize = 50;

    // Convert title to kebab-case for branch name
    let slug: String = title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    // Truncate on character boundaries. A title can contain multibyte
    // characters even though the generated slug is otherwise ASCII today.
    let slug: String = slug
        .chars()
        .take(MAX_SLUG_CHARS)
        .collect::<String>()
        .trim_end_matches('-')
        .to_string();
    let slug = if slug.is_empty() { "update" } else { &slug };

    format!("{}/{}", identifier.to_lowercase(), slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_titles_are_truncated_without_invalid_boundaries() {
        let branch = generate_branch_name("LIN-1", &"fix ".repeat(20));
        let slug = branch.strip_prefix("lin-1/").expect("identifier prefix");
        assert!(slug.chars().count() <= 50);
        assert!(!slug.ends_with('-'));
        assert!(validate_branch_name(&branch).is_ok());
    }

    #[test]
    fn multibyte_titles_do_not_panic_when_truncated() {
        let branch = generate_branch_name("LIN-2", &"界".repeat(60));
        assert_eq!(branch, format!("lin-2/{}", "界".repeat(50)));
    }

    #[test]
    fn punctuation_only_titles_get_a_valid_fallback_slug() {
        let branch = generate_branch_name("LIN-3", "!!! --- ???");
        assert_eq!(branch, "lin-3/update");
        assert!(validate_branch_name(&branch).is_ok());
    }
}
