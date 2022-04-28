use clap::{Args, Parser, Subcommand};
use std::error::Error;

mod app;
mod ui;

#[derive(Debug, Parser)]
#[clap(name=env!("CARGO_CRATE_NAME"))]
#[clap(version=env!("CARGO_PKG_VERSION"))]
#[clap(author = "Daniel Lee")]
#[clap(about = "Multi Vehicle flight controller")]
#[clap(subcommand_required = true)]
#[clap(arg_required_else_help = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Ui(Ui),
    Generate(Generate),
    Echo(Echo),
}

#[derive(Debug, Args)]
#[clap(about = "Start FlightCTL Terminal User Interface")]
#[clap(arg_required_else_help = true)]
struct Ui {
    #[clap(help = "One or more Uri to a vechicle's MavSDK Interface")]
    #[clap(short = 'v')]
    vehicles: Vec<String>,

    #[clap(help = "One or more .plan files to direct the corresponding drones")]
    #[clap(short = 'p')]
    plans: Vec<String>,
}

#[derive(Debug, Args)]
#[clap(about = "Generate Plans for flight controller")]
#[clap(short_flag = 'g')]
#[clap(long_flag = "generate")]
#[clap(arg_required_else_help = true)]
#[clap(subcommand_required = true)]
struct Generate {
    #[clap(subcommand)]
    command: Option<GenerateCommands>,
}

#[derive(Debug, Subcommand)]
enum GenerateCommands {
    Circle(GenerateCircle),
    Square(GenerateSquare),
    Line(GenerateLine),
}

#[derive(Debug, Args)]
#[clap(about = "Create circle shape")]
#[clap(short_flag = 'c')]
struct GenerateCircle {
    #[clap(short = 'c')]
    count: u8,
}

#[derive(Debug, Args)]
#[clap(about = "Create square shape")]
struct GenerateSquare {
    #[clap(short = 'c')]
    count: u8,
}

#[derive(Debug, Args)]
#[clap(about = "Create line shape")]
struct GenerateLine {
    #[clap(short = 'c')]
    count: u8,
}

#[derive(Debug, Args)]
#[clap(short_flag = 'e')]
#[clap(long_flag = "echo")]
#[clap(about = "CLI Parser sanity check")]
#[clap(arg_required_else_help = true)]
struct Echo {
    text: String,
}

#[tokio::main]
#[allow(unreachable_code)] // ignore loop code while TUI is implemented
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Ui {} => {
            for v in
            println!()
        }
        Some(("ui", ui_matches)) => {
            // TODO: Implement TUI to
            let targets: Vec<_> = ui_matches
                .values_of("vehicles")
                .unwrap()
                .map(|s| s.to_string())
                .collect();
            let plans: Vec<_> = ui_matches
                .values_of("plans")
                .unwrap()
                .map(|s| s.to_string())
                .collect();
            loop {} // Placeholder for TUI Run code
                    // Placeholder for cleanup code / graceful exit
        }
        Some(("generate", gen_matches)) => match gen_matches.subcommand() {
            _ => unreachable!(),
        },
        Some(("echo", _)) => {
            println!("Hello World!");
        }
        // default
        _ => unreachable!(),
    }
    Ok(())
}
