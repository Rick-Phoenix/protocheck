#![allow(dead_code)]

use db::establish_connection;
use db::models::*;
use db::schema::base_stats::{self};
use db::schema::pokemons;
use db::schema::types;
use db::schema::{image_data, pokemon_types};
use diesel::prelude::*;
use macro_impl::ProtoMessage;
use serde_json;
use std::fs;

type AppResult<T> = Result<T, Box<dyn std::error::Error + 'static>>;

pub fn load_pokemons_from_json(
  file_path: &str,
) -> Result<Vec<JsonPokemon>, Box<dyn std::error::Error>> {
  let contents = fs::read_to_string(file_path)?;

  let pokemons: Vec<JsonPokemon> = serde_json::from_str(&contents)?;

  Ok(pokemons)
}

fn insert_pokemons() -> AppResult<()> {
  let json_file_path = "data/pokemon.json";

  match load_pokemons_from_json(json_file_path) {
    Ok(pokemons_data) => {
      let conn = &mut db::establish_connection();

      for p in pokemons_data {
        let new_stats = BaseStat {
          id: 0,
          hp: p.base.hp,
          attack: p.base.attack,
          defense: p.base.defense,
          special_attack: p.base.sp_attack,
          special_defense: p.base.sp_defense,
          speed: p.base.speed,
          pokemon_id: p.id,
        };

        let inserted_stats: BaseStat = diesel::insert_into(base_stats::table)
          .values(&new_stats)
          .returning(BaseStat::as_returning())
          .get_result(conn)?;

        let new_image_data = ImageData {
          id: 0,
          sprite: p.image.sprite,
          pokemon_id: p.id,
          thumbnail: p.image.thumbnail,
          hires: p.image.hires,
        };

        let inserted_img_data: ImageData = diesel::insert_into(image_data::table)
          .values(&new_image_data)
          .returning(ImageData::as_returning())
          .get_result(conn)?;

        let mut pok_data = Pokemon {
          id: p.id,
          name: p.name,
          description: p.description.clone(),
          image_data_id: inserted_img_data.id,
          base_stats_id: inserted_stats.id,
          next_evolution_id: None,
          prev_evolution_id: None,
        };

        if let Some(next_evolution) = p.evolution.next {
          let next_ev_id_str = &next_evolution[0][0];
          let next_ev_id = next_ev_id_str.parse::<i32>()?;

          pok_data.next_evolution_id = Some(next_ev_id);
        };

        if let Some(prev_evolution) = p.evolution.prev {
          let prev_ev_id_str = &prev_evolution[0];
          let prev_ev_id = prev_ev_id_str.parse::<i32>()?;

          pok_data.prev_evolution_id = Some(prev_ev_id);
        };

        diesel::insert_into(pokemons::table)
          .values(&pok_data)
          .execute(conn)
          .unwrap();

        for type_name in &p.types {
          let type_id: i32 = {
            let existing_type: Option<Type> = types::table
              .filter(types::name.eq(type_name))
              .select(Type::as_select())
              .first(conn)
              .optional()?;

            if let Some(t) = existing_type {
              t.id
            } else {
              let new_type = Type {
                id: 0,
                name: type_name.clone(),
              };

              let inserted_type: Type = diesel::insert_into(types::table)
                .values(&new_type)
                .returning(Type::as_returning())
                .get_result(conn)?;

              inserted_type.id
            }
          };

          let new_poke_type = PokemonType {
            pokemon_id: p.id,
            type_id,
          };

          diesel::insert_into(pokemon_types::table)
            .values(&new_poke_type)
            .execute(conn)?;
        }
      }
    }
    Err(e) => {
      eprintln!("Error loading or parsing JSON file: {}", e);
    }
  };

  Ok(())
}

#[derive(Debug)]
struct PokeData {
  pub pokemon: Pokemon,
  pub images: ImageData,
  pub stats: BaseStat,
  pub types: Vec<String>,
}

fn select_pokemon() -> AppResult<()> {
  let conn = &mut establish_connection();
  let poke_data: Pokemon = pokemons::table
    .filter(pokemons::id.eq(1))
    .select(Pokemon::as_select())
    .get_result(conn)?;

  let base_stats = BaseStat::belonging_to(&poke_data)
    .select(BaseStat::as_select())
    .get_result(conn)?;
  let img_data = ImageData::belonging_to(&poke_data)
    .select(ImageData::as_select())
    .get_result(conn)?;

  let poke_types = PokemonType::belonging_to(&poke_data)
    .inner_join(types::table)
    .select(types::name)
    .load::<String>(conn)?;

  let complete_data = PokeData {
    pokemon: poke_data,
    stats: base_stats,
    types: poke_types,
    images: img_data,
  };
  println!("Complete data: {:#?}", complete_data);
  Ok(())
}

fn test_macro() -> AppResult<()> {
  let conn = &mut establish_connection();
  let poke_data: Pokemon = pokemons::table
    .filter(pokemons::id.eq(1))
    .select(Pokemon::as_select())
    .get_result(conn)?;

  let fields = poke_data.get_fields();
  println!("Fields: {:#?}", fields);
  Ok(())
}

fn complex_queries() -> AppResult<()> {
  let conn = &mut establish_connection();
  let pokemon_with_types = pokemon_types::table
    .inner_join(types::table)
    .inner_join(pokemons::table);

  let grass_pokemons = pokemon_with_types
    .filter(types::name.eq("Grass"))
    .select(pokemons::name)
    .limit(5)
    .load::<String>(conn)?;
  println!("Grass pokemons: {:#?}", grass_pokemons);

  let fire_pokemons = pokemon_with_types
    .filter(types::name.eq("Fire"))
    .select(pokemons::name)
    .limit(5)
    .load::<String>(conn)?;
  println!("Fire pokemons: {:#?}", fire_pokemons);

  Ok(())
}

fn main() -> AppResult<()> {
  // insert_pokemons()?;
  // select_pokemon()?;
  // complex_queries()?;
  test_macro()?;
  Ok(())
}
