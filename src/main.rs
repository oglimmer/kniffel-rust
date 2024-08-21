#[macro_use]
extern crate rocket;
mod game_logic;

use lazy_static::lazy_static;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;
use std::sync::{Mutex, MutexGuard};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::{OpenApi, ToSchema};
use game_logic::BookingType;
use game_logic::GameService;


lazy_static! {
    // Use Mutex to ensure thread safety
    static ref GAME_SERVICE: Mutex<GameService> = Mutex::new(GameService::new());
}


#[derive(Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
struct CreateGameRequest {
    player_names: Vec<String>,
}


#[derive(Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
struct DiceRollRequest {
    dice_to_keep: Vec<i32>,
}

#[derive(Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
struct BookRollRequest {
    booking_type: String,
}


#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct PlayerData {
    name: String,

    score: i32,
}

impl PlayerData {
    pub fn new(_name: String, _score: i32) -> Self {
        PlayerData {
            name: _name.to_string(),
            score: _score,
        }
    }
}


#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct GameResponse {
    game_id: String,
    player_data: Vec<PlayerData>,
    current_player_name: String,
    state: String,
    used_booking_types: Vec<String>,
    available_booking_types: Vec<String>,
    dice_rolls: [i32; 5],
    roll_round: i32,
}

#[utoipa::path(
    request_body = CreateGameRequest,
    responses(
            (status = 200, description = "Create a new game", body = GameResponse)
    )
)]
#[post("/api/v1/game/<_..>", rank = 5, format = "json", data = "<player_request>")]
fn post_player_names(player_request: Json<CreateGameRequest>) -> Json<GameResponse> {
    let player_names = &player_request.player_names;

    let game_service = GAME_SERVICE.lock().unwrap();

    let game = game_service.create_game(player_names);

    create_return_data(game.game_id, game_service)
}

#[utoipa::path(
    responses(
            (status = 200, description = "Retrieve a game", body = GameResponse)
    )
)]
#[get("/api/v1/game/<game_id>")]
fn get_player_names(game_id: String) -> Json<GameResponse> {
    let game_service = GAME_SERVICE.lock().unwrap();

    create_return_data(game_id, game_service)
}

#[utoipa::path(
    request_body = DiceRollRequest,
    responses(
            (status = 200, description = "(Re)-roll the dice", body = GameResponse)
    )
)]
#[post("/api/v1/game/<game_id>/roll", format = "json", data = "<dice_roll_request>")]
fn roll(game_id: String, dice_roll_request: Json<DiceRollRequest>) -> Json<GameResponse> {
    let game_service = GAME_SERVICE.lock().unwrap();

    let mut game = game_service.get_game(game_id.to_string()).unwrap();

    game.re_roll_dice(&dice_roll_request.dice_to_keep);

    game_service.update_game(&game);

    create_return_data(game_id, game_service)
}

#[utoipa::path(
    request_body = BookRollRequest,
    responses(
            (status = 200, description = "Book a dice roll to score", body = GameResponse)
    )
)]
#[post("/api/v1/game/<game_id>/book", format = "json", data = "<dice_book_request>")]
fn book(game_id: String, dice_book_request: Json<BookRollRequest>) -> Json<GameResponse> {
    let game_service = GAME_SERVICE.lock().unwrap();

    let mut game = game_service.get_game(game_id.to_string()).unwrap();

    game.book_dice_roll(BookingType::from_str(&dice_book_request.booking_type.to_string()).unwrap());

    game_service.update_game(&game);

    create_return_data(game_id, game_service)
}

fn create_return_data(game_id: String, game_service: MutexGuard<GameService>) -> Json<GameResponse> {
    let game = game_service.get_game(game_id.to_string()).unwrap();

    // Define the full set of BookingType
    let full_set: HashSet<BookingType> = [
        BookingType::Ones,
        BookingType::Twos,
        BookingType::Threes,
        BookingType::Fours,
        BookingType::Fives,
        BookingType::Sixes,
        BookingType::ThreeOfAKind,
        BookingType::FourOfAKind,
        BookingType::FullHouse,
        BookingType::SmallStraight,
        BookingType::LargeStraight,
        BookingType::Kniffel,
        BookingType::Chance,
    ]
        .iter()
        .cloned()
        .collect();

    let player = game.players.get(&game.current_player);

    let inverted_set: HashSet<BookingType> = full_set
        .difference(&player.unwrap().used_booking_types)
        .cloned()
        .collect();

    Json(GameResponse {
        game_id: game.game_id,
        player_data: game.players.values().map(|e| PlayerData::new(e.name.clone(), e.score)).collect(),
        current_player_name: game.current_player,
        state: game.state.to_string().to_uppercase(),
        used_booking_types: player.unwrap().used_booking_types.iter().map(|bt| bt.to_string()).collect(),
        available_booking_types: inverted_set.iter().map(|bt| bt.to_string()).collect(),
        dice_rolls: game.dice_rolls,
        roll_round: game.roll_round,
    })
}


#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cors = rocket_cors::CorsOptions { ..Default::default() }.to_cors()?;

    #[derive(OpenApi)]
    #[openapi(
        paths(
            post_player_names,
            get_player_names,
            roll,
            book
        ),
        components(
            schemas(GameResponse, CreateGameRequest, DiceRollRequest, BookRollRequest, PlayerData)
        ),
    )]
    struct ApiDoc;

    let _ = rocket::build()
        .configure(rocket::Config::figment()
            .merge(("port", 8080))
            .merge(("address", "0.0.0.0")))
        .mount("/", routes![post_player_names, get_player_names, roll, book])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
