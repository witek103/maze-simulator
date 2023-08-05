mod communication;
mod context;
mod distance_sensors;
mod engine;
mod environment;
mod maze;
mod mazefile;
mod panel;
mod position;
mod runner;
mod simulator;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use mazefile::Mazefile;
use simulator::MazeSimulator;

pub const COLS: usize = 16;
pub const ROWS: usize = 16;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to mazefile with the map
    #[arg(short, long)]
    mazefile: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let maze = Mazefile::<ROWS, COLS>::load(args.mazefile)?.parse()?;

    MazeSimulator::run(maze)
}
