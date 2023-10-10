use clap::Parser;
use std::str::FromStr;
use strum::VariantNames;

mod types;
use types::{Cast, Faction, Robot};

#[derive(Debug, Clone, clap::Args)]
struct Options {
  #[arg(short, long, default_value = "cast.yaml")]
  input_file: std::path::PathBuf,
  #[arg(short, long)]
  output_file: Option<std::path::PathBuf>,
}

#[derive(Debug, Clone, clap::Parser)]
struct Command {
  #[command(subcommand)]
  subcommand: Commands,
  #[command(flatten)]
  options: Options,
}

#[derive(Debug, Clone, clap::Subcommand)]
enum Commands {
  Create,
  Show,
}

fn main() -> anyhow::Result<()> {
  let command = Command::parse();
  match command.subcommand {
    Commands::Create => create(command.options),
    Commands::Show => show(command.options),
  }
}

fn create(options: Options) -> anyhow::Result<()> {
  let name: String = dialoguer::Input::new().with_prompt("Name").interact()?;
  let commander = dialoguer::Confirm::new()
    .with_prompt("Commander?")
    .default(false)
    .interact()?;
  let subcommander = dialoguer::Confirm::new()
    .with_prompt("Subcommander?")
    .default(false)
    .interact()?;
  let has_faction = dialoguer::Confirm::new()
    .with_prompt("Faction?")
    .default(true)
    .interact()?;
  let faction = if has_faction {
    let index = dialoguer::Select::new()
      .with_prompt("Faction")
      .items(Faction::VARIANTS)
      .interact()?;
    let faction = Faction::from_str(Faction::VARIANTS[index])?;
    Some(faction)
  } else {
    None
  };
  let robot = Robot {
    commander,
    faction,
    name,
    subcommander,
  };
  let yaml = serde_yaml::to_string(&robot)?;
  match options.output_file {
    None => println!("{yaml}"),
    Some(filename) => std::fs::write(filename, yaml)?,
  }
  Ok(())
}

fn show(options: Options) -> anyhow::Result<()> {
  let yaml = std::fs::read_to_string(options.input_file)?;
  let cast: Cast = serde_yaml::from_str(&yaml)?;
  for robot in cast.robots {
    match robot.faction {
      Some(Faction::Autobots) => println!("🤖 {}", robot.name),
      Some(Faction::Decepticons) => println!("👾 {}", robot.name),
      Some(Faction::Dinobots) => println!("🦖 {}", robot.name),
      None => println!("? {}", robot.name),
    }
  }
  Ok(())
}
