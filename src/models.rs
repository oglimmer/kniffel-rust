use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Game {
    pub id: i32,
    pub game_id: String,
    pub roll_round: i32,
    pub stage: String,
    pub dice_rolls: String,
    pub current_player: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::players)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Player {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub score: i32,
    pub used_booking_types: String,
}

#[derive(QueryableByName, Debug)]
#[diesel(table_name = crate::schema::last_insert)]
// #[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct LastInsertId {
    pub last_insert_id: i32,
}