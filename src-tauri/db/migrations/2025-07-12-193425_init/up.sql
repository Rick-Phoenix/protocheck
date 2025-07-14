CREATE TABLE pokemons (
id integer NOT NULL PRIMARY KEY autoincrement,
name text NOT NULL,
next_evolution_id integer,
prev_evolution_id integer,
description text NOT NULL,
image_data_id integer NOT NULL,
base_stats_id integer NOT NULL,
FOREIGN KEY (next_evolution_id) REFERENCES pokemons (id),
FOREIGN KEY (prev_evolution_id) REFERENCES pokemons (id),
FOREIGN KEY (image_data_id) REFERENCES image_data (id),
FOREIGN KEY (base_stats_id) REFERENCES base_stats (id)
) ;

CREATE TABLE base_stats (
id integer not null PRIMARY KEY autoincrement,
hp integer NOT NULL,
attack integer NOT NULL,
defense integer NOT NULL,
special_attack integer NOT NULL,
special_defense integer NOT NULL,
speed integer NOT NULL,
pokemon_id integer NOT NULL,
FOREIGN KEY (pokemon_id) REFERENCES pokemons (id)
) ;

CREATE TABLE image_data (
id integer not null PRIMARY KEY autoincrement,
sprite text NOT NULL,
thumbnail text NOT NULL,
hires text NOT NULL,
pokemon_id integer NOT NULL,
FOREIGN KEY (pokemon_id) REFERENCES pokemons (id)
) ;

CREATE TABLE types (
id integer not null PRIMARY KEY autoincrement,
name text NOT NULL
) ;

CREATE TABLE pokemon_types (
pokemon_id integer NOT NULL,
type_id integer NOT NULL,
FOREIGN KEY (pokemon_id) REFERENCES pokemons (id),
FOREIGN KEY (type_id) REFERENCES types (id),
PRIMARY KEY (pokemon_id, type_id)
) ;
