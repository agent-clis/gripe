use colored::Colorize;

use crate::config::resolve_schema;

pub fn run(json: bool) -> Result<(), String> {
    let schema = resolve_schema()?;

    if json {
        let output = serde_json::to_string_pretty(&schema)
            .map_err(|e| format!("Failed to serialize: {}", e))?;
        println!("{}", output);
    } else {
        if let Some(repo) = &schema.repo {
            println!("{}: {}", "Repo".bold(), repo);
        }
        if !schema.labels.is_empty() {
            println!("{}: {}", "Labels".bold(), schema.labels.join(", "));
        }
        if let Some(tmpl) = &schema.title_template {
            println!("{}: {}", "Title template".bold(), tmpl);
        }
        println!();
        println!("{}", "Fields:".bold());
        for field in &schema.fields {
            let req = if field.required {
                " (required)".red().to_string()
            } else {
                " (optional)".dimmed().to_string()
            };
            println!(
                "  {} [{}]{}",
                field.display_label().cyan(),
                format!("{:?}", field.field_type).to_lowercase(),
                req
            );
            if !field.options.is_empty() {
                println!("    options: {}", field.options.join(", "));
            }
        }
    }

    Ok(())
}
