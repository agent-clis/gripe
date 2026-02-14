use crate::types::{FieldDefinition, FieldType, GripeSchema, AutomatedPolicy};

pub fn default_schema() -> GripeSchema {
    GripeSchema {
        repo: None,
        automated: AutomatedPolicy::Allow,
        labels: vec!["feedback".to_string()],
        title_template: Some("[{tool}] {summary}".to_string()),
        fields: vec![
            FieldDefinition {
                id: "tool".to_string(),
                label: Some("Tool Name".to_string()),
                field_type: FieldType::Input,
                required: true,
                options: vec![],
                default: None,
            },
            FieldDefinition {
                id: "summary".to_string(),
                label: Some("Summary".to_string()),
                field_type: FieldType::Input,
                required: true,
                options: vec![],
                default: None,
            },
            FieldDefinition {
                id: "expected".to_string(),
                label: Some("Expected Behavior".to_string()),
                field_type: FieldType::Textarea,
                required: true,
                options: vec![],
                default: None,
            },
            FieldDefinition {
                id: "actual".to_string(),
                label: Some("Actual Behavior".to_string()),
                field_type: FieldType::Textarea,
                required: true,
                options: vec![],
                default: None,
            },
            FieldDefinition {
                id: "severity".to_string(),
                label: Some("Severity".to_string()),
                field_type: FieldType::Select,
                required: false,
                options: vec![
                    "low".to_string(),
                    "medium".to_string(),
                    "high".to_string(),
                    "critical".to_string(),
                ],
                default: Some("medium".to_string()),
            },
            FieldDefinition {
                id: "context".to_string(),
                label: Some("Additional Context".to_string()),
                field_type: FieldType::Textarea,
                required: false,
                options: vec![],
                default: None,
            },
        ],
    }
}
