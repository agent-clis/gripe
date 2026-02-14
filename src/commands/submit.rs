use std::collections::HashMap;
use std::io::{self, Read};

use colored::Colorize;
use dialoguer::{Input, Select};

use crate::body_formatter::{format_body, format_title};
use crate::config::resolve_schema;
use crate::gh;
use crate::types::{AutomatedPolicy, FieldType, GripeSchema};

pub fn run(
    json_str: Option<String>,
    stdin: bool,
    dry_run: bool,
    output_json: bool,
    repo_override: Option<String>,
    field_args: Vec<String>,
) -> Result<(), String> {
    let schema = resolve_schema()?;
    let is_interactive;

    // Determine input mode and collect values
    let values = if let Some(ref j) = json_str {
        is_interactive = false;
        parse_json_input(j)?
    } else if stdin {
        is_interactive = false;
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| format!("Failed to read stdin: {}", e))?;
        parse_json_input(buf.trim())?
    } else if !field_args.is_empty() {
        is_interactive = false;
        parse_kv_args(&field_args)?
    } else {
        is_interactive = true;
        interactive_prompt(&schema)?
    };

    // Validate required fields
    validate_fields(&schema, &values)?;

    // Resolve repo
    let repo = repo_override
        .or_else(|| schema.repo.clone())
        .ok_or_else(|| {
            "No repo specified. Use --repo, set repo in gripe.yaml, or run from a git repo."
                .to_string()
        })?;

    let title = format_title(schema.title_template.as_deref(), &values);
    let body = format_body(&schema, &values);

    if dry_run {
        if output_json {
            let output = serde_json::json!({
                "repo": repo,
                "title": title,
                "body": body,
                "labels": schema.labels,
                "fields": values,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&output).map_err(|e| format!("JSON error: {}", e))?
            );
        } else {
            println!("{}", "--- Dry Run ---".yellow().bold());
            println!("{}: {}", "Repo".bold(), repo);
            println!("{}: {}", "Title".bold(), title);
            if !schema.labels.is_empty() {
                println!("{}: {}", "Labels".bold(), schema.labels.join(", "));
            }
            println!();
            println!("{}", body);
        }
        return Ok(());
    }

    // Check gh availability
    gh::check_gh_available()?;

    // Check robots policy for non-interactive submissions
    if !is_interactive {
        match gh::check_robots(&repo)? {
            AutomatedPolicy::Deny => {
                return Err(format!(
                    "This repository does not accept automated feedback. See: https://github.com/{}/blob/main/gripe.yaml",
                    repo
                ));
            }
            AutomatedPolicy::Allow => {}
        }
    }

    let result = gh::create_issue(&repo, &title, &body, &schema.labels)?;

    if output_json {
        let output = serde_json::json!({
            "url": result.url,
            "number": result.number,
            "repo": repo,
            "title": title,
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&output).map_err(|e| format!("JSON error: {}", e))?
        );
    } else {
        println!("{} Issue created: {}", "✓".green(), result.url.underline());
    }

    Ok(())
}

fn parse_json_input(input: &str) -> Result<HashMap<String, String>, String> {
    let map: HashMap<String, serde_json::Value> =
        serde_json::from_str(input).map_err(|e| format!("Invalid JSON: {}", e))?;

    Ok(map
        .into_iter()
        .map(|(k, v)| {
            let s = match v {
                serde_json::Value::String(s) => s,
                other => other.to_string(),
            };
            (k, s)
        })
        .collect())
}

fn parse_kv_args(args: &[String]) -> Result<HashMap<String, String>, String> {
    let mut values = HashMap::new();
    for arg in args {
        let (key, val) = arg
            .split_once('=')
            .ok_or_else(|| format!("Invalid field argument '{}'. Expected key=value.", arg))?;
        values.insert(key.to_string(), val.to_string());
    }
    Ok(values)
}

fn validate_fields(schema: &GripeSchema, values: &HashMap<String, String>) -> Result<(), String> {
    let missing: Vec<&str> = schema
        .fields
        .iter()
        .filter(|f| f.required)
        .filter(|f| values.get(&f.id).map(|v| v.is_empty()).unwrap_or(true))
        .map(|f| f.display_label())
        .collect();

    if !missing.is_empty() {
        return Err(format!("Missing required fields: {}", missing.join(", ")));
    }

    Ok(())
}

fn interactive_prompt(schema: &GripeSchema) -> Result<HashMap<String, String>, String> {
    let mut values = HashMap::new();

    for field in &schema.fields {
        let label = format!(
            "{}{}",
            field.display_label(),
            if field.required { " *" } else { "" }
        );

        let value = match field.field_type {
            FieldType::Select if !field.options.is_empty() => {
                let default_idx = field
                    .default
                    .as_ref()
                    .and_then(|d| field.options.iter().position(|o| o == d))
                    .unwrap_or(0);

                let selection = Select::new()
                    .with_prompt(&label)
                    .items(&field.options)
                    .default(default_idx)
                    .interact()
                    .map_err(|e| format!("Prompt error: {}", e))?;

                field.options[selection].clone()
            }
            _ => {
                let mut prompt = Input::<String>::new().with_prompt(&label);

                if let Some(ref def) = field.default {
                    prompt = prompt.default(def.clone());
                }

                if !field.required {
                    prompt = prompt.allow_empty(true);
                }

                prompt
                    .interact_text()
                    .map_err(|e| format!("Prompt error: {}", e))?
            }
        };

        if !value.is_empty() {
            values.insert(field.id.clone(), value);
        }
    }

    Ok(values)
}
