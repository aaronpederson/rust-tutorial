#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct Cast {
  pub robots: Vec<Robot>,
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct Robot {
  pub name: String,
  pub faction: Option<Faction>,
  pub commander: bool,
  pub subcommander: bool,
  pub strength: u8,
  pub intelligence: u8,
  pub speed: u8,
  pub endurance: u8,
  pub rank: u8,
  pub courage: u8,
  pub firepower: u8,
  pub skill: u8,
}

#[derive(
  Debug, Clone, serde::Deserialize, serde::Serialize, strum::EnumVariantNames, strum::EnumString,
)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Faction {
  Autobots,
  Decepticons,
  Dinobots,
}
