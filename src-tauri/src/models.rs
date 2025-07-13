use crate::schema::*;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JsonPokemon {
  pub id: i32,
  pub name: String,
  #[serde(rename = "type")]
  pub types: Vec<String>,
  pub base: BaseStats,
  pub description: String,
  pub evolution: EvolutionData,
  pub image: ImageData,
}

#[derive(Debug, Deserialize)]
pub struct BaseStats {
  #[serde(rename = "HP")]
  pub hp: i32,
  #[serde(rename = "Attack")]
  pub attack: i32,
  #[serde(rename = "Defense")]
  pub defense: i32,
  #[serde(rename = "Sp. Attack")]
  pub sp_attack: i32,
  #[serde(rename = "Sp. Defense")]
  pub sp_defense: i32,
  #[serde(rename = "Speed")]
  pub speed: i32,
}

#[derive(Debug, Deserialize)]
pub struct EvolutionData {
  pub next: Option<Vec<Vec<String>>>,
  pub prev: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ImageData {
  pub sprite: String,
  pub thumbnail: String,
  pub hires: String,
}

// DB MODELS

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = pokemons)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DbPokemon {
  pub id: i32,
  pub name: String,
  pub next_evolution_id: Option<i32>,
  pub prev_evolution_id: Option<i32>,
  pub description: String,
  pub image_data_id: i32,
  pub base_stats_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = base_stats)]
pub struct NewBaseStats {
  pub hp: i32,
  pub attack: i32,
  pub defense: i32,
  pub special_attack: i32,
  pub special_defense: i32,
  pub speed: i32,
  pub pokemon_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = base_stats)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BaseStatsFromDb {
  pub id: i32,
  pub hp: i32,
  pub attack: i32,
  pub defense: i32,
  pub special_attack: i32,
  pub special_defense: i32,
  pub speed: i32,
  pub pokemon_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = image_data)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ImageDataFromDb {
  pub id: i32,
  pub sprite: String,
  pub thumbnail: String,
  pub hires: String,
  pub pokemon_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = image_data)]
pub struct NewImageData {
  pub sprite: String,
  pub thumbnail: String,
  pub hires: String,
  pub pokemon_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = types)]
pub struct NewType {
  pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = types)]
pub struct TypeFromDb {
  pub id: i32,
  pub name: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = pokemon_types)]
pub struct PokemonType {
  pub pokemon_id: i32,
  pub type_id: i32,
}
