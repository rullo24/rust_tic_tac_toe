use rand::Rng;

pub struct XAndOValues{
    pub top_left: char,
    pub top_middle: char,
    pub top_right: char,
    pub middle_left: char,
    pub middle_middle: char,
    pub middle_right: char,
    pub bottom_left: char,
    pub bottom_middle: char,
    pub bottom_right: char
}

impl Default for XAndOValues {
    fn default() -> Self {
        XAndOValues {
            top_left: ' ',
            top_middle: ' ',
            top_right: ' ',
            middle_left: ' ',
            middle_middle: ' ',
            middle_right: ' ',
            bottom_left: ' ',
            bottom_middle: ' ',
            bottom_right: ' ',
        }
    }
}

pub struct GameSettings{
    pub game_won: bool,
    pub winner_symbol: char,
    pub players_turn: char,
    pub stalemate_achieved: bool
}

impl Default for GameSettings {
    fn default() -> Self {
        let possible_player_icon: Vec<char> = vec!['X', 'O'];
        let random_index_player_icon = rand::thread_rng().gen_range(0..possible_player_icon.len());

        GameSettings {
            game_won: false,
            winner_symbol: ' ',
            players_turn: possible_player_icon[random_index_player_icon], // Chooses a random player to start
            stalemate_achieved: false
        }
    }
}