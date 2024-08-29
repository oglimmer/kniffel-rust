use crate::game_logic::KniffelGame;
use crate::models::{Game, LastInsertId, Player};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error;
use diesel::{insert_into, sql_query, update, Connection, MysqlConnection, QueryDsl, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::env;
use std::sync::Mutex;
use lazy_static::lazy_static;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn get_connection_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    dotenv().ok();

    let url = env::var("MYSQL_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

lazy_static! {
    static ref POOL: Mutex<Pool<ConnectionManager<MysqlConnection>>> = Mutex::new(get_connection_pool());
}

pub(crate) fn init() {
    let conn = &mut POOL.lock().unwrap().get().unwrap();
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run database migrations");
}

pub(crate) fn persist_new_game(kniffel_game: KniffelGame) {
    let connection = &mut POOL.lock().unwrap().get().unwrap();
    connection.transaction::<_, Error, _>(|con| {
        insert_game_to_db(con, &kniffel_game);

        let game_id = get_last_id(con);

        insert_players_to_db(con, &kniffel_game, game_id);

        Ok(())
    }).expect("Failed to commit game");
}

fn get_last_id(con: &mut MysqlConnection) -> i32 {
    let last_id_result = sql_query("SELECT LAST_INSERT_ID() as last_insert_id")
        .load::<LastInsertId>(con);

    let vec_last_id = last_id_result.expect("failed to get vec for last_insert_id");

    let last_id = vec_last_id.get(0).expect("failed to get item for last_insert_id");

    last_id.last_insert_id
}

fn insert_game_to_db(con: &mut MysqlConnection, kniffel_game: &KniffelGame) {
    use crate::schema::games::dsl::*;
    let _ = insert_into(games)
        .values((
            game_id.eq(kniffel_game.game_id.clone()),
            roll_round.eq(kniffel_game.roll_round),
            stage.eq(kniffel_game.state.to_string()),
            dice_rolls.eq(kniffel_game.dice_rolls
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")),
            current_player.eq(kniffel_game.current_player.to_string())
        ))
        .execute(con);
}

fn insert_players_to_db(con: &mut MysqlConnection, kniffel_game: &KniffelGame, game_id_param: i32) {
    use crate::schema::players::dsl::*;

    kniffel_game.players.values().for_each(|player_var| {
        let _ = insert_into(players)
            .values((
                game_id.eq(game_id_param),
                name.eq(player_var.name.to_string()),
                score.eq(0),
                used_booking_types.eq("")
            ))
            .execute(con);
    });
}

pub(crate) fn load_game_from_persistent_store(game_id_param: String) -> Option<KniffelGame> {
    let connection = &mut POOL.lock().unwrap().get().unwrap();
    let option_game = load_game(connection, game_id_param);

    let game = option_game.expect("failed to load game");

    let game_id_id = game.id;

    let players = load_players(connection, game_id_id);

    Some(KniffelGame::from_db(game, players.as_slice()))
}

fn load_game(connection: &mut MysqlConnection, game_id_param: String) -> Option<Game> {
    use crate::schema::games::dsl::*;

    let result = games
        .select(Game::as_select())
        .limit(1)
        .filter(game_id.eq(game_id_param))
        .load(connection)
        .expect("failed to load game");

    result.into_iter().next()
}

fn load_players(connection: &mut MysqlConnection, game_id_param: i32) -> Vec<Player> {
    use crate::schema::players::dsl::*;
    let result_players = players
        .select(Player::as_select())
        .filter(game_id.eq(game_id_param))
        .load(connection)
        .expect("failed to load players");

    result_players
}

pub(crate) fn update_game_to_persistent_store(kniffel_game: &KniffelGame) {
    let connection = &mut POOL.lock().unwrap().get().unwrap();
    {
        connection.transaction::<_, Error, _>(|con| {
            let option_game = load_game(con, kniffel_game.game_id.to_string());
            let game = option_game.expect("failed to load game");

            update_game_to_db(con, game.id, kniffel_game);
            update_players_to_db(con, game.id, kniffel_game);

            Ok(())
        }).expect("Failed to commit update game");
    }
}

fn update_game_to_db(con: &mut MysqlConnection, game_id_param: i32, kniffel_game: &KniffelGame) {
    use crate::schema::games::dsl::*;
    let _ = update(games)
        .filter(id.eq(game_id_param))
        .set((
            roll_round.eq(kniffel_game.roll_round),
            stage.eq(kniffel_game.state.to_string()),
            dice_rolls.eq(kniffel_game.dice_rolls
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")),
            current_player.eq(kniffel_game.current_player.to_string())
        ))
        .execute(con);
}
fn update_players_to_db(con: &mut MysqlConnection, game_id_param: i32, kniffel_game: &KniffelGame) {
    use crate::schema::players::dsl::*;

    kniffel_game.players.values().for_each(|player_var| {
        let planer_name = player_var.name.to_string();
        let _ = update(players)
            .filter(name.eq(planer_name))
            .filter(game_id.eq(game_id_param))
            .set((
                score.eq(player_var.score),
                used_booking_types.eq(player_var.used_booking_types.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","))
            ))
            .execute(con);
    });
}