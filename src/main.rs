mod trafic;
use std::collections::HashMap;

use crate::trafic::*;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
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
    let client = &reqwest::Client::new();
    let trafic_service = TraficService::init(client);
    match &cli.command {
        Commands::Trafic(trafic) => match &trafic.command {
            TraficSubCommands::Overview(_) => match trafic_service.get_incidents().await {
                Ok(incidents) => {
                    let mut reasons: HashMap<String, u16> = HashMap::new();
                    for incident in incidents.iter() {
                        let key = &incident.severity;
                        let value: u16 = if reasons.contains_key(key) {
                            reasons.get(key).unwrap() + 1
                        } else {
                            1
                        };
                        reasons.insert(key.clone(), value);
                    }
                    println!("{:?}", reasons);
                }
                Err(reason) => println!("{:?}", reason),
            },
            TraficSubCommands::Incidents(_) => match trafic_service.get_incidents().await {
                Ok(incidents) => {
                    for incident in incidents.iter() {
                        println!("{}", &incident.toDisplayString());
                    }
                }
                Err(reason) => println!("{:?}", reason),
            },
        },
    }
}
