CREATE TABLE games (
  id INTEGER AUTO_INCREMENT PRIMARY KEY,
  game_id VARCHAR(255) NOT NULL,
  roll_round INTEGER not null,
  stage VARCHAR(255) NOT NULL,
  dice_rolls VARCHAR(255) NOT NULL,
  current_player VARCHAR(255) NOT NULL
);

CREATE TABLE players (
  id INTEGER AUTO_INCREMENT PRIMARY KEY,
  game_id integer not null,
  name VARCHAR(255) NOT NULL,
  score integer not null,
  used_booking_types VARCHAR(255) NOT NULL
);
