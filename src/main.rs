extern crate core;

use crate::generate::{CircleMission, LineMission, ShapeMission, SquareMission};
use clap::{Args, Parser, Subcommand};
use std::error::Error;
use std::path::Path;
use std::time::Duration;

mod app;
mod generate;
mod run_mode;
mod ui;

#[derive(Debug, Parser)]
#[clap(name=env!("CARGO_CRATE_NAME"))]
#[clap(version=env!("CARGO_PKG_VERSION"))]
#[clap(author = "Daniel Lee")]
#[clap(about = "Multi Vehicle flight controller")]
#[clap(long_about = "FlightCTL handles 2 modes of execution:\n\
                    Run Mode - This mode takes 1 or more pairs of plans and vehicles
                        ")]
#[clap(subcommand_required = true)]
#[clap(arg_required_else_help = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(about = "Start FlightCTL Terminal User Interface in UI mode")]
    #[clap(arg_required_else_help = true)]
    UI {},
    Run(Run),
    Generate(Generate),
    Echo(Echo),
}

#[derive(Debug, Args)]
#[clap(about = "Start FlightCTL Terminal User Interface in run mode")]
#[clap(arg_required_else_help = true)]
struct Run {
    #[clap(help = "One or more Uri to a vechicle's MavSDK Interface")]
    #[clap(short = 'v')]
    vehicles: Vec<String>,

    #[clap(help = "One or more .plan files to direct the corresponding drones")]
    #[clap(short = 'p')]
    plans: Vec<String>,
}

#[derive(Debug, Args)]
#[clap(about = "Generate Plans in QGroundControl format for run mode")]
#[clap(short_flag = 'g')]
#[clap(long_flag = "generate")]
#[clap(arg_required_else_help = true)]
#[clap(subcommand_required = true)]
struct Generate {
    #[clap(subcommand)]
    command: GenerateCommands,

    #[clap(short = 'p')]
    #[clap(help = "Path to a directory to save the plan files")]
    path: String,
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
    #[clap(help = "Number of vehicles used to create the circle")]
    count: u8,

    #[clap(short = 'r')]
    #[clap(help = "Radius of the circle shape in meters")]
    radius: u8,

    #[clap(long = "slat")]
    #[clap(help = "The Drone's starting Latitude")]
    start_lat: f64,

    #[clap(long = "slon")]
    #[clap(help = "The Drone's starting Longitude")]
    start_lon: f64,

    #[clap(long = "tlat")]
    #[clap(help = "The shape's location Latitude")]
    target_lat: f64,

    #[clap(long = "tlon")]
    #[clap(help = "The shape's location Longitude")]
    target_lon: f64,

    #[clap(long = "talt")]
    #[clap(help = "The shape's location altitude")]
    target_alt: u8,

    #[clap(short = 'h')]
    #[clap(help = "How long to hold the shape in seconds")]
    hold_sec: u8,
}

impl GenerateCircle {
    fn to_circle_mission(&self) -> CircleMission {
        CircleMission::new(
            self.count,
            self.radius,
            self.start_lat,
            self.start_lon,
            self.target_lat,
            self.target_lon,
            self.target_alt,
            self.hold_sec,
        )
    }
}

#[derive(Debug, Args)]
#[clap(about = "Create square shape with 4 drones")]
#[clap(short_flag = 's')]
struct GenerateSquare {
    #[clap(short = 'w')]
    #[clap(help = "Width of each side of the square shape in meters")]
    width: u8,

    #[clap(long = "slat")]
    #[clap(help = "The Drone's starting Latitude")]
    start_lat: f64,

    #[clap(long = "slon")]
    #[clap(help = "The Drone's starting Longitude")]
    start_lon: f64,

    #[clap(long = "tlat")]
    #[clap(help = "The shape's location Latitude")]
    target_lat: f64,

    #[clap(long = "tlon")]
    #[clap(help = "The shape's location Longitude")]
    target_lon: f64,

    #[clap(long = "talt")]
    #[clap(help = "The shape's location altitude")]
    target_alt: u8,

    #[clap(short = 'h')]
    #[clap(help = "How long to hold the shape in seconds")]
    hold_sec: u8,
}

impl GenerateSquare {
    fn to_square_mission(&self) -> SquareMission {
        SquareMission::new(
            self.width,
            self.start_lat,
            self.start_lon,
            self.target_lat,
            self.target_lon,
            self.target_alt,
            self.hold_sec,
        )
    }
}

#[derive(Debug, Args)]
#[clap(about = "Create line shape with 3 drones")]
#[clap(short_flag = 'l')]
struct GenerateLine {
    #[clap(short = 'w')]
    #[clap(help = "Width to create the line shape in meters")]
    width: u8,

    #[clap(short = 'a')]
    #[clap(help = "Angle to create the line shape in radians relative to the earth's longitude")]
    angle: f64,

    #[clap(long = "slat")]
    #[clap(help = "The Drone's starting Latitude")]
    start_lat: f64,

    #[clap(long = "slon")]
    #[clap(help = "The Drone's starting Longitude")]
    start_lon: f64,

    #[clap(long = "tlat")]
    #[clap(help = "The shape's location Latitude")]
    target_lat: f64,

    #[clap(long = "tlon")]
    #[clap(help = "The shape's location Longitude")]
    target_lon: f64,

    #[clap(long = "talt")]
    #[clap(help = "The shape's location altitude")]
    target_alt: u8,

    #[clap(short = 'h')]
    #[clap(help = "How long to hold the shape in seconds")]
    hold_sec: u8,
}

impl GenerateLine {
    fn to_line_mission(&self) -> LineMission {
        LineMission::new(
            self.width,
            self.angle,
            self.start_lat,
            self.start_lon,
            self.target_lat,
            self.target_lon,
            self.target_alt,
            self.hold_sec,
        )
    }
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
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::UI {} => {
            app::run(Duration::from_millis(200), None, false);
        }
        Commands::Run(run) => {
            if run.vehicles.len() != run.plans.len() {
                panic!("Error, vehicle and plan count mismatch")
            }
            let mut sets = vec![];
            for i in 0..run.vehicles.len() {
                println!("Found {} {}", run.vehicles[i], run.plans[i]);
                sets.push((run.vehicles[i].clone(), run.plans[i].clone()));
            }
            app::run(Duration::from_millis(200), Some(sets), true);
        }
        Commands::Generate(generate) => {
            let gen_cmd = generate.command;
            match gen_cmd {
                GenerateCommands::Circle(circle) => {
                    println!("Generate circle shape with {} vehicles", circle.count);
                    circle
                        .to_circle_mission()
                        .write_mission_to_disk(Path::new(generate.path.as_str()))?;
                }
                GenerateCommands::Square(square) => {
                    println!(
                        "Generating square shape with sides length {} meters",
                        square.width
                    );
                    square
                        .to_square_mission()
                        .write_mission_to_disk(Path::new(generate.path.as_str()))?;
                }
                GenerateCommands::Line(line) => {
                    println!("Generate line shape at angle {} radians", line.angle);
                    line.to_line_mission()
                        .write_mission_to_disk(Path::new(generate.path.as_str()))?;
                }
            }
        }
        Commands::Echo(echo) => {
            println!("Echo: {}", echo.text)
        }
    }
    Ok(())
}
