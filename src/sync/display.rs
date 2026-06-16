use super::DiffResult;
use colored::Colorize;

pub fn print_diff(diff: &DiffResult) {
    if diff.additions.is_empty() && diff.updates.is_empty() && diff.deletions.is_empty() {
        println!(
            "{}",
            "No changes detected. Configuration is up to date.".green()
        );
        return;
    }

    for add in &diff.additions {
        let name = add
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unnamed");
        let impl_type = add
            .get("implementation")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");
        println!("{}", format!("+ [Add] {} ({}):", name, impl_type).green());
        if let Some(fields) = add.get("fields").and_then(|v| v.as_array()) {
            for f in fields {
                let f_name = f.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let f_val = f.get("value").map(|v| v.to_string()).unwrap_or_default();
                println!("{}", format!("    + {}: {}", f_name, f_val).green());
            }
        }
    }

    for update in &diff.updates {
        let name = update
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unnamed");
        let impl_type = update
            .get("implementation")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");
        println!(
            "{}",
            format!("~ [Update] {} ({}):", name, impl_type).yellow()
        );
        if let Some(fields) = update.get("fields").and_then(|v| v.as_array()) {
            for f in fields {
                let f_name = f.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let f_val = f.get("value").map(|v| v.to_string()).unwrap_or_default();
                println!("{}", format!("    ~ {}: {}", f_name, f_val).yellow());
            }
        }
    }

    for del in &diff.deletions {
        let name = del
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unnamed");
        let impl_type = del
            .get("implementation")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");
        println!("{}", format!("- [Delete] {} ({}):", name, impl_type).red());
    }
}
