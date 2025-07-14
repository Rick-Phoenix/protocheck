// @generated automatically by Diesel CLI.

diesel::table! {
    base_stats (id) {
        id -> Integer,
        hp -> Integer,
        attack -> Integer,
        defense -> Integer,
        special_attack -> Integer,
        special_defense -> Integer,
        speed -> Integer,
        pokemon_id -> Integer,
    }
}

diesel::table! {
    image_data (id) {
        id -> Integer,
        sprite -> Text,
        thumbnail -> Text,
        hires -> Text,
        pokemon_id -> Integer,
    }
}

diesel::table! {
    pokemon_types (pokemon_id, type_id) {
        pokemon_id -> Integer,
        type_id -> Integer,
    }
}

diesel::table! {
    pokemons (id) {
        id -> Integer,
        name -> Text,
        next_evolution_id -> Nullable<Integer>,
        prev_evolution_id -> Nullable<Integer>,
        description -> Text,
        image_data_id -> Integer,
        base_stats_id -> Integer,
    }
}

diesel::table! {
    types (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(pokemon_types -> pokemons (pokemon_id));
diesel::joinable!(pokemon_types -> types (type_id));

diesel::allow_tables_to_appear_in_same_query!(
    base_stats,
    image_data,
    pokemon_types,
    pokemons,
    types,
);
