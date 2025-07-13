use diesel::prelude::*;
use serde_json;
use std::fs;
use tauri_app_lib::models::*;
use tauri_app_lib::schema::base_stats::{self};
use tauri_app_lib::schema::{image_data, pokemon_types};
use tauri_app_lib::*;

pub fn load_pokemons_from_json(
  file_path: &str,
) -> Result<Vec<JsonPokemon>, Box<dyn std::error::Error>> {
  let contents = fs::read_to_string(file_path)?;

  let pokemons: Vec<JsonPokemon> = serde_json::from_str(&contents)?;

  Ok(pokemons)
}

// evolutions
// if let Some(next_evolution) = p.evolution.next {
//   let next_ev_id_str = &next_evolution[0][0];
//   let next_ev_id = next_ev_id_str.parse::<i32>()?;
//
//   diesel::update(pokemons::table)
//     .filter(pokemons::id.eq(p.id))
//     .set(pokemons::next_evolution_id.eq(next_ev_id))
//     .execute(conn)?;
// };

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let json_file_path = "data/pokemon.json";

  match load_pokemons_from_json(json_file_path) {
    Ok(pokemons_data) => {
      let conn = &mut establish_connection();

      use self::schema::pokemons;
      use self::schema::types::dsl::*;

      for p in pokemons_data {
        let new_stats = NewBaseStats {
          hp: p.base.hp,
          attack: p.base.attack,
          defense: p.base.defense,
          special_attack: p.base.sp_attack,
          special_defense: p.base.sp_defense,
          speed: p.base.speed,
          pokemon_id: p.id,
        };

        let inserted_stats: BaseStatsFromDb = diesel::insert_into(base_stats::table)
          .values(&new_stats)
          .returning(BaseStatsFromDb::as_returning())
          .get_result(conn)?;

        let new_image_data = NewImageData {
          sprite: p.image.sprite,
          pokemon_id: p.id,
          thumbnail: p.image.thumbnail,
          hires: p.image.hires,
        };

        let inserted_img_data: ImageDataFromDb = diesel::insert_into(image_data::table)
          .values(&new_image_data)
          .returning(ImageDataFromDb::as_returning())
          .get_result(conn)?;

        let pok_data = DbPokemon {
          id: p.id,
          name: p.name,
          description: p.description.clone(),
          image_data_id: inserted_img_data.id,
          base_stats_id: inserted_stats.id,
          next_evolution_id: None,
          prev_evolution_id: None,
        };

        diesel::insert_into(pokemons::table)
          .values(&pok_data)
          .execute(conn)
          .unwrap();

        for type_name in &p.types {
          let type_id: i32 = {
            let existing_type: Option<TypeFromDb> = types
              .filter(name.eq(type_name))
              .select(TypeFromDb::as_select())
              .first(conn)
              .optional()?;

            if let Some(t) = existing_type {
              t.id
            } else {
              let new_type = NewType {
                name: type_name.clone(),
              };

              let inserted_type: TypeFromDb = diesel::insert_into(types)
                .values(&new_type)
                .returning(TypeFromDb::as_returning())
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
