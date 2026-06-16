use crate::config::parse_key_val;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(
    name = "configuratarr",
    version,
    about = "Declarative configuration sync engine for the *arr stack"
)]
pub struct Cli {
    #[arg(
        long,
        global = true,
        help = "Starr application base URL override (e.g. http://127.0.0.1:7878)"
    )]
    pub url: Option<String>,

    #[arg(long, global = true, help = "Starr application API key override")]
    pub api_key: Option<String>,

    #[arg(
        long,
        global = true,
        help = "Wait for the target application to become online before execution"
    )]
    pub wait: bool,

    #[arg(
        long,
        global = true,
        default_value = "30",
        help = "Timeout in seconds to wait for system status check to pass"
    )]
    pub wait_timeout: u64,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(about = "Synchronize all configured applications from the configuration file")]
    Sync(SyncCmd),
    #[command(about = "Manage Radarr configuration")]
    Radarr(AppCmd),
    #[command(about = "Manage Sonarr configuration")]
    Sonarr(AppCmd),
    #[command(about = "Manage Prowlarr configuration")]
    Prowlarr(AppCmd),
    #[command(about = "Manage Lidarr configuration")]
    Lidarr(AppCmd),
    #[command(about = "Manage Readarr configuration")]
    Readarr(AppCmd),
}

#[derive(Args, Debug, Clone)]
#[group(required = true, multiple = false)]
pub struct PlanApply {
    #[arg(long, help = "Calculate diff and print dry-run")]
    pub plan: bool,

    #[arg(long, help = "Commit the changes to the server")]
    pub apply: bool,
}

#[derive(Args, Debug, Clone)]
pub struct SyncCmd {
    #[arg(long, default_value = "configuratarr.yaml")]
    pub config: String,

    #[arg(long)]
    pub prune: bool,

    #[command(flatten)]
    pub plan_apply: PlanApply,

    #[arg(long, requires = "apply")]
    pub auto_approve: bool,
}

#[derive(Args, Debug, Clone)]
pub struct AppCmd {
    #[command(subcommand)]
    pub command: AppSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum AppSubcommands {
    #[command(about = "Synchronize specific application settings from configuration file")]
    Sync(SyncCmd),
    #[command(about = "Check connection and online status")]
    Status,
    #[command(about = "Manage Download Clients")]
    DownloadClient(ResourceCmd),
    #[command(about = "Manage Indexers")]
    Indexer(ResourceCmd),
    #[command(about = "Manage Root Folders")]
    RootFolder(ResourceCmd),
    #[command(about = "Manage Quality Profiles")]
    QualityProfile(ResourceCmd),
    #[command(about = "Manage Custom Formats")]
    CustomFormat(ResourceCmd),
    #[command(about = "Manage Metadata Profiles")]
    MetadataProfile(ResourceCmd),
    #[command(about = "Manage Release Profiles")]
    ReleaseProfile(ResourceCmd),
    #[command(about = "Manage UI Configuration")]
    Ui(SingletonCmd),
    #[command(about = "Manage Naming Configuration")]
    Naming(SingletonCmd),
    #[command(about = "Manage Media Management Configuration")]
    MediaManagement(SingletonCmd),
}

#[derive(Args, Debug, Clone)]
pub struct SingletonCmd {
    #[command(subcommand)]
    pub command: SingletonSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SingletonSubcommands {
    #[command(about = "Show current configuration")]
    Show,
    #[command(about = "Update configuration values")]
    Update {
        #[arg(
            long,
            value_parser = parse_key_val,
            value_name = "KEY=VALUE",
            action = clap::ArgAction::Append,
            help = "Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light)."
        )]
        field: Vec<(String, serde_json::Value)>,
    },
}

#[derive(Args, Debug, Clone)]
pub struct ResourceCmd {
    #[command(subcommand)]
    pub command: ResourceSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ResourceSubcommands {
    #[command(about = "List all server-configured resources")]
    List,
    #[command(about = "Delete a resource by name")]
    Delete {
        name: String,
    },
    #[command(about = "Add a new resource configuration dynamically")]
    Add {
        #[arg(
            long,
            value_parser = parse_key_val,
            value_name = "KEY=VALUE",
            action = clap::ArgAction::Append,
            help = "Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true)."
        )]
        field: Vec<(String, serde_json::Value)>,
    },
    #[command(about = "Update an existing resource configuration or add it if it does not exist")]
    Update {
        #[arg(
            long,
            value_parser = parse_key_val,
            value_name = "KEY=VALUE",
            action = clap::ArgAction::Append,
            help = "Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true)."
        )]
        field: Vec<(String, serde_json::Value)>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_docs() {
        let markdown = clap_markdown::help_markdown::<Cli>();
        if std::env::var("GENERATE_DOCS").is_ok() {
            let path = std::path::Path::new("docs/COMMANDS.md");
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::write(path, markdown).unwrap();
        } else {
            let existing = std::fs::read_to_string("docs/COMMANDS.md").unwrap_or_default();
            assert_eq!(
                existing.trim(),
                markdown.trim(),
                "docs/COMMANDS.md is out of date. Run with `GENERATE_DOCS=1 cargo test` to update it."
            );
        }
    }
}
