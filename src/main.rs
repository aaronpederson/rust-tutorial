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
  Download,
}

fn main() -> anyhow::Result<()> {
  let command = Command::parse();
  match command.subcommand {
    Commands::Create => create(command.options),
    Commands::Show => show(command.options),
    Commands::Download => download(command.options),
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
  let strength = dialoguer::Input::new()
    .with_prompt("Strength?")
    .default(5)
    .interact()?;
  let intelligence = dialoguer::Input::new()
    .with_prompt("Intelligence?")
    .default(5)
    .interact()?;
  let speed = dialoguer::Input::new()
    .with_prompt("Speed?")
    .default(5)
    .interact()?;
  let endurance = dialoguer::Input::new()
    .with_prompt("Endurance?")
    .default(5)
    .interact()?;
  let rank = dialoguer::Input::new()
    .with_prompt("Rank?")
    .default(5)
    .interact()?;
  let courage = dialoguer::Input::new()
    .with_prompt("Courage?")
    .default(5)
    .interact()?;
  let firepower = dialoguer::Input::new()
    .with_prompt("Firepower?")
    .default(5)
    .interact()?;
  let skill = dialoguer::Input::new()
    .with_prompt("Skill?")
    .default(5)
    .interact()?;
  let robot = Robot {
    commander,
    courage,
    endurance,
    faction,
    firepower,
    intelligence,
    name,
    rank,
    skill,
    speed,
    strength,
    subcommander,
    ..Default::default()
  };
  let yaml = serde_yaml::to_string(&robot)?;
  match options.output_file {
    None => println!("\n{yaml}"),
    Some(filename) => std::fs::write(filename, yaml)?,
  }
  Ok(())
}

fn download(_options: Options) -> anyhow::Result<()> {
  let url = "https://www.ntfa.net/ntfa/techspecs/index.php?cat=Gen1&group=AutoOZ&char=Optimus_Prime";
  let html = ureq::get(url).call()?.into_string()?;
    println!("{html}");
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
