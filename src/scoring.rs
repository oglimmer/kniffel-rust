


/// Calculates the score for a specific value (1 to 6) based on its occurrences in `dice_rolls`.
pub fn get_score_1_to_6(dice_rolls: &[i32; 5], value_to_score: i32) -> i32 {
    dice_rolls
        .iter() // Iterate over the dice_rolls array
        .filter(|&&roll| roll == value_to_score) // Filter for the value_to_score
        .count() as i32 * value_to_score // Multiply the count by value_to_score
}

/// Calculates the score for X of a kind, where X is specified by `value_to_score`.
pub fn get_score_for_x_of_a_kind(dice_rolls: &[i32; 5], value_to_score: i32) -> i32 {
    // Create an array to count occurrences of each value (1 through 6)
    let mut counts = [0; 6];

    // Count the occurrences of each dice value
    for &roll in dice_rolls {
        if roll >= 1 && roll <= 6 {
            counts[(roll - 1) as usize] += 1;
        }
    }

    // Check if any value appears exactly `value_to_score` times
    if counts.iter().any(|&count| count >= value_to_score) {
        // If such a value exists, return the sum of all dice rolls
        dice_rolls.iter().sum()
    } else {
        // Otherwise, return 0
        0
    }
}

/// Calculates the score for a Full House. Returns 25 if it is a Full House, otherwise 0.
pub fn get_score_full_house(dice_rolls: &[i32; 5]) -> i32 {
    // Create an array to count occurrences of each value (1 through 6)
    let mut counts = [0; 6];

    // Count the occurrences of each dice value
    for &roll in dice_rolls {
        if roll >= 1 && roll <= 6 {
            counts[(roll - 1) as usize] += 1;
        }
    }

    // Check if there's a Full House (one 3-count and one 2-count)
    let has_three_of_a_kind = counts.iter().any(|&count| count == 3);
    let has_two_of_a_kind = counts.iter().any(|&count| count == 2);

    if has_three_of_a_kind && has_two_of_a_kind {
        25
    } else {
        0
    }
}

/// Calculates the score for a Small Straight. Returns 30 if it is a Small Straight, otherwise 0.
pub fn get_score_small_straight(dice_rolls: &[i32; 5]) -> i32 {
    // Create a set to store unique dice rolls
    let mut unique_rolls = dice_rolls.to_vec();
    unique_rolls.sort_unstable();
    unique_rolls.dedup();  // Remove duplicates

    // Check for the presence of any of the valid Small Straight sequences
    let small_straights = [
        [1, 2, 3, 4], // 1-2-3-4
        [2, 3, 4, 5], // 2-3-4-5
        [3, 4, 5, 6], // 3-4-5-6
    ];

    for straight in small_straights.iter() {
        if straight.iter().all(|&num| unique_rolls.contains(&num)) {
            return 30;
        }
    }

    0
}

/// Calculates the score for a Large Straight. Returns 40 if it is a Large Straight, otherwise 0.
pub fn get_score_large_straight(dice_rolls: &[i32; 5]) -> i32 {
    // Create a vector to store the dice rolls
    let mut sorted_rolls = dice_rolls.to_vec();
    sorted_rolls.sort_unstable();  // Sort the rolls
    sorted_rolls.dedup();  // Remove duplicates

    // There are only two valid sequences for a Large Straight
    let large_straights = [
        [1, 2, 3, 4, 5],  // Sequence 1-2-3-4-5
        [2, 3, 4, 5, 6],  // Sequence 2-3-4-5-6
    ];

    // Check if the sorted rolls match either of the valid Large Straight sequences
    for straight in large_straights.iter() {
        if sorted_rolls == straight {
            return 40;
        }
    }

    0
}

/// Calculates the score for a Kniffel (Yahtzee). Returns 50 if it is a Kniffel, otherwise 0.
pub fn get_score_kniffel(dice_rolls: &[i32; 5]) -> i32 {
    // Check if all dice rolls are the same
    if dice_rolls.iter().all(|&roll| roll == dice_rolls[0]) {
        50
    } else {
        0
    }
}

/// Calculates the score for Chance. Returns the sum of all dice.
pub fn get_score_chance(dice_rolls: &[i32; 5]) -> i32 {
    dice_rolls.iter().sum()
}
