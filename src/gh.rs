use std::process::Command;

use colored::Colorize;

use crate::types::AutomatedPolicy;

pub struct IssueResult {
    pub url: String,
    pub number: u64,
}

pub fn check_gh_available() -> Result<(), String> {
    Command::new("gh")
        .args(["--version"])
        .output()
        .map_err(|_| "gh CLI not found. Install it: https://cli.github.com".to_string())?;
    Ok(())
}

/// Check if the target repo allows automated submissions.
/// Fetches gripe.yaml from the target repo via gh api.
pub fn check_robots(repo: &str) -> Result<AutomatedPolicy, String> {
    let output = Command::new("gh")
        .args([
            "api",
            &format!("repos/{}/contents/gripe.yaml", repo),
            "--jq",
            ".content",
        ])
        .output()
        .map_err(|e| format!("Failed to run gh: {}", e))?;

    if !output.status.success() {
        // No gripe.yaml in target repo — allow by default
        return Ok(AutomatedPolicy::Allow);
    }

    let encoded = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if encoded.is_empty() {
        return Ok(AutomatedPolicy::Allow);
    }

    // GitHub API returns base64-encoded content (with possible newlines)
    let cleaned: String = encoded.chars().filter(|c| !c.is_whitespace()).collect();
    let decoded =
        base64_decode(&cleaned).map_err(|e| format!("Failed to decode content: {}", e))?;

    match serde_yaml::from_str::<serde_yaml::Value>(&decoded) {
        Ok(val) => {
            if let Some(automated) = val.get("automated").and_then(|v| v.as_str()) {
                if automated == "deny" {
                    return Ok(AutomatedPolicy::Deny);
                }
            }
            Ok(AutomatedPolicy::Allow)
        }
        Err(_) => Ok(AutomatedPolicy::Allow),
    }
}

/// Simple base64 decoder (avoids adding a dependency for this one use)
fn base64_decode(input: &str) -> Result<String, String> {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    fn val(c: u8) -> Result<u8, String> {
        if let Some(pos) = TABLE.iter().position(|&b| b == c) {
            Ok(pos as u8)
        } else if c == b'=' {
            Ok(0)
        } else {
            Err(format!("Invalid base64 character: {}", c as char))
        }
    }

    let bytes = input.as_bytes();
    let mut output = Vec::with_capacity(bytes.len() * 3 / 4);

    for chunk in bytes.chunks(4) {
        if chunk.len() < 4 {
            break;
        }
        let a = val(chunk[0])?;
        let b = val(chunk[1])?;
        let c = val(chunk[2])?;
        let d = val(chunk[3])?;

        output.push((a << 2) | (b >> 4));
        if chunk[2] != b'=' {
            output.push((b << 4) | (c >> 2));
        }
        if chunk[3] != b'=' {
            output.push((c << 6) | d);
        }
    }

    String::from_utf8(output).map_err(|e| format!("Invalid UTF-8: {}", e))
}

pub fn create_issue(
    repo: &str,
    title: &str,
    body: &str,
    labels: &[String],
) -> Result<IssueResult, String> {
    // Try with labels first, retry without if labels don't exist on the repo
    let result = try_create_issue(repo, title, body, labels)?;
    match result {
        Ok(r) => Ok(r),
        Err(stderr) if stderr.contains("label") && stderr.contains("not found") => {
            eprintln!(
                "{} Labels {:?} not found on repo, creating issue without labels",
                "warning:".yellow(),
                labels
            );
            try_create_issue(repo, title, body, &[])?
                .map_err(|e| format!("gh issue create failed: {}", e))
        }
        Err(stderr) => Err(format!("gh issue create failed: {}", stderr)),
    }
}

fn try_create_issue(
    repo: &str,
    title: &str,
    body: &str,
    labels: &[String],
) -> Result<Result<IssueResult, String>, String> {
    let mut args = vec![
        "issue".to_string(),
        "create".to_string(),
        "--repo".to_string(),
        repo.to_string(),
        "--title".to_string(),
        title.to_string(),
        "--body".to_string(),
        body.to_string(),
    ];

    for label in labels {
        args.push("--label".to_string());
        args.push(label.to_string());
    }

    let output = Command::new("gh")
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to run gh: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Ok(Err(stderr));
    }

    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let number = url
        .rsplit('/')
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    Ok(Ok(IssueResult { url, number }))
}
