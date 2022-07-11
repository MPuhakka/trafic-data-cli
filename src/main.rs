mod commands;
pub mod data_models;
pub mod services;
pub mod utilities;

use crate::services::*;
use clap::{Args, Parser, Subcommand};
use commands::{IncidentCommandHandler, TraficIncidentList, TraficOverview};
use utilities::TablePrinter;
#[derive(Parser)]
#[clap(author = "by Mika Puhakka", about = "CLI for reading data in tampere region from various open APIs", version, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Trafic(Trafic),
}

#[derive(Args)]
struct Trafic {
    #[clap(subcommand)]
    command: TraficSubCommands,
}

#[derive(Subcommand)]
enum TraficSubCommands {
    Overview(TraficOverview),
    Incidents(TraficIncidentList),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let http_client = &reqwest::Client::new();
    match &cli.command {
        Commands::Trafic(trafic) => {
            let handler = IncidentCommandHandler {
                trafic_service: &TraficService::init(http_client),
                table_printer: &TablePrinter { padding: Some(3) },
            };

            match &trafic.command {
                TraficSubCommands::Overview(_) => handler.incident_overview().await,
                TraficSubCommands::Incidents(command) => handler.list_incidents(command).await,
            }
        }
    }
}
