use std::env;
use std::path::{Path, PathBuf};

use crate::defaults::default_schema;
use crate::git::{detect_repo, find_git_root};
use crate::github_templates::load_github_templates;
use crate::types::GripeSchema;

/// Walk up from `start` looking for gripe.yaml
fn find_gripe_yaml(start: &Path) -> Option<PathBuf> {
    let mut dir = start.to_path_buf();
    loop {
        let candidate = dir.join("gripe.yaml");
        if candidate.is_file() {
            return Some(candidate);
        }
        if !dir.pop() {
            return None;
        }
    }
}

fn load_gripe_yaml(path: &Path) -> Result<GripeSchema, String> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    serde_yaml::from_str(&contents)
        .map_err(|e| format!("Failed to parse {}: {}", path.display(), e))
}

/// Resolve schema using fallback chain:
/// 1. gripe.yaml (walk up from cwd)
/// 2. .github/ISSUE_TEMPLATE/*.yml in git root
/// 3. Built-in default
pub fn resolve_schema() -> Result<GripeSchema, String> {
    let cwd = env::current_dir().map_err(|e| format!("Cannot get cwd: {}", e))?;

    // 1. Try gripe.yaml
    if let Some(yaml_path) = find_gripe_yaml(&cwd) {
        let mut schema = load_gripe_yaml(&yaml_path)?;
        if schema.repo.is_none() {
            schema.repo = detect_repo(&cwd);
        }
        return Ok(schema);
    }

    // 2. Try .github/ISSUE_TEMPLATE
    if let Some(git_root) = find_git_root(&cwd) {
        let templates_dir = git_root.join(".github").join("ISSUE_TEMPLATE");
        if templates_dir.is_dir() {
            if let Some(mut schema) = load_github_templates(&templates_dir) {
                if schema.repo.is_none() {
                    schema.repo = detect_repo(&cwd);
                }
                return Ok(schema);
            }
        }
    }

    // 3. Built-in default
    let mut schema = default_schema();
    schema.repo = detect_repo(&cwd);
    Ok(schema)
}
