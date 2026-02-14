use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GripeSchema {
    #[serde(default)]
    pub repo: Option<String>,
    #[serde(default = "default_automated")]
    pub automated: AutomatedPolicy,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub title_template: Option<String>,
    pub fields: Vec<FieldDefinition>,
}

fn default_automated() -> AutomatedPolicy {
    AutomatedPolicy::Allow
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AutomatedPolicy {
    Allow,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub id: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default = "default_field_type")]
    #[serde(rename = "type")]
    pub field_type: FieldType,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub default: Option<String>,
}

fn default_field_type() -> FieldType {
    FieldType::Input
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    Input,
    Textarea,
    Select,
}

impl FieldDefinition {
    pub fn display_label(&self) -> &str {
        self.label.as_deref().unwrap_or(&self.id)
    }
}
