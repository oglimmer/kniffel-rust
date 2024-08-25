// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Integer,
        #[max_length = 255]
        game_id -> Varchar,
        roll_round -> Integer,
        #[max_length = 255]
        stage -> Varchar,
        #[max_length = 255]
        dice_rolls -> Varchar,
        #[max_length = 255]
        current_player -> Varchar,
    }
}

diesel::table! {
    players (id) {
        id -> Integer,
        game_id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        score -> Integer,
        #[max_length = 255]
        used_booking_types -> Varchar,
    }
}

diesel::table! {
    last_insert (last_insert_id) {
        last_insert_id -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    games,
    players,
);
