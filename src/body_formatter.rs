use std::collections::HashMap;

use crate::types::GripeSchema;

/// Render field values into a Markdown issue body.
pub fn format_body(schema: &GripeSchema, values: &HashMap<String, String>) -> String {
    let mut sections = Vec::new();

    for field in &schema.fields {
        let value = values.get(&field.id).map(|s| s.as_str()).unwrap_or("");
        if value.is_empty() {
            continue;
        }

        let label = field.display_label();
        sections.push(format!("### {}\n\n{}", label, value));
    }

    sections.join("\n\n")
}

/// Render the issue title from the template.
pub fn format_title(template: Option<&str>, values: &HashMap<String, String>) -> String {
    match template {
        Some(tmpl) => {
            let mut title = tmpl.to_string();
            for (key, val) in values {
                title = title.replace(&format!("{{{}}}", key), val);
            }
            // Clean up any unreplaced placeholders
            title = title.replace("{", "").replace("}", "");
            title
        }
        None => values
            .get("summary")
            .cloned()
            .unwrap_or_else(|| "Feedback".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_title_with_template() {
        let mut values = HashMap::new();
        values.insert("tool".to_string(), "vim".to_string());
        values.insert("summary".to_string(), "cursor jumps".to_string());

        let title = format_title(Some("[{tool}] {summary}"), &values);
        assert_eq!(title, "[vim] cursor jumps");
    }

    #[test]
    fn test_format_title_no_template() {
        let mut values = HashMap::new();
        values.insert("summary".to_string(), "something broke".to_string());

        let title = format_title(None, &values);
        assert_eq!(title, "something broke");
    }
}
