use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::types::{AutomatedPolicy, FieldDefinition, FieldType, GripeSchema};

#[derive(Debug, Deserialize)]
struct GitHubTemplate {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    labels: Option<Vec<String>>,
    #[serde(default)]
    body: Vec<GitHubTemplateField>,
}

#[derive(Debug, Deserialize)]
struct GitHubTemplateField {
    #[serde(rename = "type")]
    field_type: String,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    attributes: Option<GitHubFieldAttributes>,
    #[serde(default)]
    validations: Option<GitHubFieldValidations>,
}

#[derive(Debug, Deserialize)]
struct GitHubFieldAttributes {
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    options: Option<Vec<String>>,
    #[serde(default)]
    value: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubFieldValidations {
    #[serde(default)]
    required: Option<bool>,
}

/// Load the first usable GitHub issue template from the directory.
pub fn load_github_templates(templates_dir: &Path) -> Option<GripeSchema> {
    let entries = fs::read_dir(templates_dir).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str());
        if ext != Some("yml") && ext != Some("yaml") {
            continue;
        }

        let contents = fs::read_to_string(&path).ok();
        let contents = contents.as_deref()?;

        if let Ok(template) = serde_yaml::from_str::<GitHubTemplate>(contents) {
            let fields = convert_template_fields(&template.body);
            if fields.is_empty() {
                continue;
            }

            return Some(GripeSchema {
                repo: None,
                automated: AutomatedPolicy::Allow,
                labels: template.labels.unwrap_or_default(),
                title_template: template.name.map(|n| format!("[{}] {{summary}}", n)),
                fields,
            });
        }
    }

    None
}

fn convert_template_fields(body: &[GitHubTemplateField]) -> Vec<FieldDefinition> {
    body.iter()
        .filter_map(|field| {
            let id = field.id.as_ref()?;
            if field.field_type == "markdown" {
                return None;
            }

            let attrs = field.attributes.as_ref();
            let field_type = match field.field_type.as_str() {
                "input" => FieldType::Input,
                "textarea" => FieldType::Textarea,
                "dropdown" => FieldType::Select,
                _ => FieldType::Input,
            };

            Some(FieldDefinition {
                id: id.clone(),
                label: attrs.and_then(|a| a.label.clone()),
                field_type,
                required: field
                    .validations
                    .as_ref()
                    .and_then(|v| v.required)
                    .unwrap_or(false),
                options: attrs
                    .and_then(|a| a.options.clone())
                    .unwrap_or_default(),
                default: attrs.and_then(|a| a.value.clone()),
            })
        })
        .collect()
}
