mod commands;
pub mod trafic;

use crate::trafic::*;
use clap::{Args, Parser, Subcommand};
use commands::{incident_overview, list_incidents};

#[derive(Parser)]
#[clap(author = "by Mika Puhakka", about = "CLI for reading trafic data in tampere region from various open APIs", version, long_about = None)]
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
#[derive(Args)]
struct TraficOverview {}

#[derive(Args)]
struct TraficIncidentList {}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let http_client = &reqwest::Client::new();
    let trafic_service = TraficService::init(http_client);
    match &cli.command {
        Commands::Trafic(trafic) => match &trafic.command {
            TraficSubCommands::Overview(_) => incident_overview(&trafic_service).await,
            TraficSubCommands::Incidents(_) => list_incidents(&trafic_service).await,
        },
    }
}
