use clap::{Arg, Command};
use manager::Manager;
mod manager;

#[tokio::main]
#[allow(unreachable_code)] // ignore loop code while TUI is implemented
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new(env!("CARGO_CRATE_NAME"))
        .about("Multi Vehicle flight controller")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Daniel Lee")
        .subcommand(
            Command::new("ui")
                .about("Start flightctl Terminal User Interface")
                .arg(
                    Arg::new("vehicles")
                        .multiple_values(true)
                        .help("One or more Uri to a vechicle's MavSDK Interface"),
                ),
        )
        .subcommand(
            Command::new("run")
                .short_flag('r')
                .long_flag("run")
                .about("Run flight controller on one or more drones")
                .arg(
                    Arg::new("vehicles")
                        .required(true)
                        .multiple_values(true)
                        .help("One or more Uri to a vechicle's MavSDK Interface"),
                ),
        )
        .subcommand(
            Command::new("echo")
                .short_flag('e')
                .long_flag("echo")
                .about("Sanity check cli parser"),
        )
        // space to add more subcommands in the future
        .get_matches();

    match matches.subcommand() {
        Some(("ui", ui_matches)) => {
            // TODO: Implement TUI to
            let targets: Vec<_> = ui_matches
                .values_of("vehicles")
                .unwrap()
                .map(|s| s.to_string())
                .collect();
            let mgr: Manager = Manager::new();
            mgr.add_targets(targets);
            loop {} // Placeholder for TUI Run code
                    // Placeholder for cleanup code / graceful exit
        }
        Some(("run", run_matches)) => {
            let targets: Vec<_> = run_matches
                .values_of("vehicles")
                .unwrap()
                .map(|s| s.to_string())
                .collect();
            let mgr: Manager = Manager::new();
            mgr.add_targets(targets);
            let data = mgr.get_stats().await?;
        }
        Some(("echo", _)) => {
            println!("Hello World!");
        }
        // default
        _ => unreachable!(),
    }
    Ok(())
}
