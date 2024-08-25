
use rand::Rng;
use rocket::serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;
use crate::models::{Game, Player};
use crate::scoring::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BookingType {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    ThreeOfAKind,
    FourOfAKind,
    FullHouse,
    SmallStraight,
    LargeStraight,
    Kniffel,
    Chance,
}

impl fmt::Display for BookingType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format the enum variant as SCREAMING_SNAKE_CASE
        write!(f, "{}", match self {
            BookingType::Ones => "ONES",
            BookingType::Twos => "TWOS",
            BookingType::Threes => "THREES",
            BookingType::Fours => "FOURS",
            BookingType::Fives => "FIVES",
            BookingType::Sixes => "SIXES",
            BookingType::ThreeOfAKind => "THREE_OF_A_KIND",
            BookingType::FourOfAKind => "FOUR_OF_A_KIND",
            BookingType::FullHouse => "FULL_HOUSE",
            BookingType::SmallStraight => "SMALL_STRAIGHT",
            BookingType::LargeStraight => "LARGE_STRAIGHT",
            BookingType::Kniffel => "KNIFFEL",
            BookingType::Chance => "CHANCE",
        })
    }
}

impl FromStr for BookingType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ONES" => Ok(BookingType::Ones),
            "TWOS" => Ok(BookingType::Twos),
            "THREES" => Ok(BookingType::Threes),
            "FOURS" => Ok(BookingType::Fours),
            "FIVES" => Ok(BookingType::Fives),
            "SIXES" => Ok(BookingType::Sixes),
            "THREE_OF_A_KIND" => Ok(BookingType::ThreeOfAKind),
            "FOUR_OF_A_KIND" => Ok(BookingType::FourOfAKind),
            "FULL_HOUSE" => Ok(BookingType::FullHouse),
            "SMALL_STRAIGHT" => Ok(BookingType::SmallStraight),
            "LARGE_STRAIGHT" => Ok(BookingType::LargeStraight),
            "KNIFFEL" => Ok(BookingType::Kniffel),
            "CHANCE" => Ok(BookingType::Chance),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameState {
    /// The player has to select dice they want to keep. Check the round if this is the first or second re-roll phase.
    Roll,

    /// The player has to book the dice on the table into one category.
    Book,

    /// The game has ended
    Ended,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameState::Roll => write!(f, "Roll"),
            GameState::Book => write!(f, "Book"),
            GameState::Ended => write!(f, "Ended"),
        }
    }
}

impl FromStr for GameState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Roll" => Ok(GameState::Roll),
            "Book" => Ok(GameState::Book),
            "Ended" => Ok(GameState::Ended),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KniffelPlayer {
    pub(crate) name: String,
    pub(crate) score: i32,
    /** each bookingType can only be used once per game */
    pub(crate) used_booking_types: HashSet<BookingType>,
}

impl KniffelPlayer {
    // Constructor
    pub fn new(name: &String) -> Self {
        KniffelPlayer {
            name: name.to_string(),
            score: 0,
            used_booking_types: HashSet::new(),
        }
    }

    // add_score
    pub fn add_score(&mut self, score: i32) {
        self.score += score;
    }

    // Method to add a booking type to used_booking_types
    pub fn use_booking_type(&mut self, booking_type: BookingType) {
        self.used_booking_types.insert(booking_type);
    }

    // Method to check if a booking type has been used
    pub fn has_used_booking_type(&self, booking_type: &BookingType) -> bool {
        self.used_booking_types.contains(booking_type)
    }

    fn from(player: &Player) -> Self {
        let used_booking_types = player
            .used_booking_types
            .split(',')
            .filter_map(|s| s.parse::<BookingType>().ok())
            .collect::<HashSet<_>>();

        KniffelPlayer {
            name: player.name.clone(),
            score: player.score,
            used_booking_types,
        }
    }
}

fn convert_players(players: &[Player]) -> HashMap<String, KniffelPlayer> {
    players.iter().map(|p| (p.name.clone(), KniffelPlayer::from(p))).collect()
}


#[derive(Debug, Clone)]
pub struct KniffelGame {
    pub(crate) players: HashMap<String, KniffelPlayer>,
    pub(crate) game_id: String,
    pub(crate) roll_round: i32,
    pub(crate) current_player: String,
    pub(crate) state: GameState,
    pub(crate) dice_rolls: [i32; 5],
}

impl KniffelGame {
    /// Creates a new KniffelGame for a list of players and starts the game by performing the first dice roll for the starting player.
    pub fn new(player_list: Vec<KniffelPlayer>) -> Self {
        let game_id = uuid::Uuid::new_v4().to_string().replace("-", "");
        let mut players = HashMap::new();

        for player in player_list.iter() {
            players.insert(player.name.clone(), player.clone());
        }

        let current_player = player_list[0].clone();
        let mut game = KniffelGame {
            players,
            game_id,
            roll_round: 0,
            current_player: current_player.name,
            state: GameState::Roll,
            dice_rolls: [0; 5],
        };
        let empty_array: [i32; 0] = [];
        game.re_roll_dice(&empty_array);
        game
    }

    pub fn from_db(game: Game, players: &[Player]) -> Self {
        let kniffel_players_map = convert_players(&players);

        let result: [i32; 5] = game.dice_rolls
            .split(',')
            .map(|s| s.parse::<i32>().unwrap()) // Parse each segment as i32
            .collect::<Vec<i32>>()
            .try_into() // Convert Vec<i32> to [i32; 5]
            .expect("Expected a list of 5 elements");

        let game = KniffelGame {
            players: kniffel_players_map,
            game_id: game.game_id.to_string(),
            roll_round: game.roll_round,
            current_player: game.current_player.to_string(),
            state: GameState::from_str(&game.stage.to_string()).unwrap(),
            dice_rolls: result,
        };
        game
    }

    /// Re-rolls all, some, or no dice.
    pub fn re_roll_dice(&mut self, dice_to_keep: &[i32]) {
        self.remove_dice(&dice_to_keep);
        let mut rng = rand::thread_rng();
        for value in self.dice_rolls.iter_mut() {
            if *value == 0 {
                *value = rng.gen_range(1..=6); // Generates a random number between 1 and 6 (inclusive)
            }
        }
        self.dice_rolls.sort();
        self.roll_round += 1;

        if self.roll_round == 3 {
            self.next_phase();
        }
    }

    /// Books the current dice into a booking type. Each booking type must only be used once.
    pub fn book_dice_roll(&mut self, booking_type: BookingType) {
        let to_add_score = self.get_to_add_score(booking_type);

        if let Some(player) = self.players.get_mut(&self.current_player) {
            if player.has_used_booking_type(&booking_type) {
                panic!("BookingType already used");
            }
            player.add_score(to_add_score);
            player.use_booking_type(booking_type);
        } else {
            println!("Player not found!");
        }

        self.next_phase();
    }

    fn next_phase(&mut self) {
        self.state = if self.state == GameState::Roll {
            GameState::Book
        } else {
            GameState::Roll
        };

        if self.state == GameState::Roll {
            let next_player = self.find_next_player();
            self.current_player = next_player.name;
            if next_player.used_booking_types.len() == 13 {
                self.state = GameState::Ended;
            } else {
                self.roll_round = 0;
                let empty_array: [i32; 0] = [];
                self.re_roll_dice(&empty_array);
            }
        }
    }

    fn remove_dice(&mut self, dice_to_keep: &[i32]) {
        // Create a frequency map for dice
        let mut freq_map = HashMap::new();
        for &num in self.dice_rolls.iter() {
            *freq_map.entry(num).or_insert(0) += 1;
        }

        // Initialize the result with 0s
        let mut result = [0; 5];

        // Iterate through values and update result based on freq_map
        for (i, &val) in dice_to_keep.iter().enumerate() {
            if let Some(count) = freq_map.get_mut(&val) {
                if *count > 0 {
                    result[i] = val;
                    *count -= 1;
                }
            }
        }

        // Copy result back to dice
        self.dice_rolls.copy_from_slice(&result);
    }

    fn find_next_player(&self) -> KniffelPlayer {
        let mut player_iterator = self.players.values();

        while let Some(player) = player_iterator.next() {
            if player.name.eq(&self.current_player) {
                let ret_obj = player_iterator.next();
                if ret_obj.is_none() {
                    return self.players.values().next().expect("player not found").clone();
                }
                return ret_obj.unwrap().clone();
            }
        }

        panic!("No next player found");
    }

    fn get_to_add_score(&self, booking_type: BookingType) -> i32 {
        match booking_type {
            BookingType::Ones => get_score_1_to_6(&self.dice_rolls, 1),
            BookingType::Twos => get_score_1_to_6(&self.dice_rolls, 2),
            BookingType::Threes => get_score_1_to_6(&self.dice_rolls, 3),
            BookingType::Fours => get_score_1_to_6(&self.dice_rolls, 4),
            BookingType::Fives => get_score_1_to_6(&self.dice_rolls, 5),
            BookingType::Sixes => get_score_1_to_6(&self.dice_rolls, 6),
            BookingType::ThreeOfAKind => get_score_for_x_of_a_kind(&self.dice_rolls, 3),
            BookingType::FourOfAKind => get_score_for_x_of_a_kind(&self.dice_rolls, 4),
            BookingType::Kniffel => get_score_kniffel(&self.dice_rolls),
            BookingType::FullHouse => get_score_full_house(&self.dice_rolls),
            BookingType::SmallStraight => get_score_small_straight(&self.dice_rolls),
            BookingType::LargeStraight => get_score_large_straight(&self.dice_rolls),
            BookingType::Chance => get_score_chance(&self.dice_rolls),
        }
    }
}
