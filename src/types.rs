#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct Cast {
  pub robots: Vec<Robot>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct Robot {
  pub name: String,
  pub faction: Option<Faction>,
  pub commander: bool,
  pub subcommander: bool,
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
