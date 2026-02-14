use std::path::Path;

use colored::Colorize;

use crate::defaults::default_schema;

pub fn run(force: bool) -> Result<(), String> {
    let path = Path::new("gripe.yaml");

    if path.exists() && !force {
        return Err("gripe.yaml already exists. Use --force to overwrite.".to_string());
    }

    let schema = default_schema();
    let yaml =
        serde_yaml::to_string(&schema).map_err(|e| format!("Failed to serialize schema: {}", e))?;

    std::fs::write(path, &yaml).map_err(|e| format!("Failed to write gripe.yaml: {}", e))?;

    println!("{} Created gripe.yaml", "✓".green());
    println!("Edit the file to customize your feedback schema.");

    Ok(())
}
