use crate::data_structures::{XAndOValues, GameSettings};
use std::io;
use std::io::Write; // Import the Write trait for the .flush() method
use std::time::Duration;
use std::thread::sleep;

pub fn check_if_won(ref_x_and_o: &XAndOValues, ref_game_settings: &mut GameSettings){

    // Checking Horizontally
    {
        // Checking if win occurs across top row
        if (*ref_x_and_o).top_left == (*ref_x_and_o).top_middle && (*ref_x_and_o).top_middle == (*ref_x_and_o).top_right && (*ref_x_and_o).top_left != ' '{
            (*ref_game_settings).game_won = true;
            (ref_game_settings).winner_symbol = (*ref_x_and_o).top_left;
            sleep(Duration::from_millis(500));
        }
        // Checking if win occurs across middle row
        else if (*ref_x_and_o).middle_left == (*ref_x_and_o).middle_middle && (*ref_x_and_o).middle_middle == (*ref_x_and_o).middle_right && (*ref_x_and_o).middle_right != ' '{
            (*ref_game_settings).game_won = true;
            (ref_game_settings).winner_symbol = (*ref_x_and_o).middle_left;
            sleep(Duration::from_millis(500));
        }
        // Checking if win occurs across bottom row
        else if (*ref_x_and_o).bottom_left == (*ref_x_and_o).bottom_middle && (*ref_x_and_o).bottom_middle == (*ref_x_and_o).bottom_right && (*ref_x_and_o).bottom_right != ' '{
            (*ref_game_settings).game_won = true;
            (ref_game_settings).winner_symbol = (*ref_x_and_o).bottom_left;
            sleep(Duration::from_millis(500));
        }
    }

    // Checking Vertically
    {
        // Checking if win occurs across top row
        if (*ref_x_and_o).top_left == (*ref_x_and_o).middle_left && (*ref_x_and_o).middle_left == (*ref_x_and_o).bottom_left && (*ref_x_and_o).bottom_left != ' '{
            (*ref_game_settings).game_won = true;
            (ref_game_settings).winner_symbol = (*ref_x_and_o).top_left;
            sleep(Duration::from_millis(500));
        }
        // Checking if win occurs across middle row
        else if (*ref_x_and_o).top_middle == (*ref_x_and_o).middle_middle && (*ref_x_and_o).middle_middle == (*ref_x_and_o).bottom_middle && (*ref_x_and_o).bottom_middle != ' '{
            (*ref_game_settings).game_won = true;
            (ref_game_settings).winner_symbol = (*ref_x_and_o).top_middle;
            sleep(Duration::from_millis(500));
        }
        // Checking if win occurs across bottom row
        else if (*ref_x_and_o).top_right == (*ref_x_and_o).middle_right && (*ref_x_and_o).middle_right == (*ref_x_and_o).bottom_right && (*ref_x_and_o).bottom_right != ' '{
            (*ref_game_settings).game_won = true;
            (ref_game_settings).winner_symbol = (*ref_x_and_o).top_right;
            sleep(Duration::from_millis(500));
        }
    }

    // Checking Diagonals
    {
        // Checking if win occurs across top row
        if (*ref_x_and_o).top_left == (*ref_x_and_o).middle_middle && (*ref_x_and_o).middle_middle == (*ref_x_and_o).bottom_right && (*ref_x_and_o).bottom_right != ' '{
            (*ref_game_settings).game_won = true;
            (ref_game_settings).winner_symbol = (*ref_x_and_o).top_left;
            sleep(Duration::from_millis(500));
        }
        // Checking if win occurs across middle row
        else if (*ref_x_and_o).top_right == (*ref_x_and_o).middle_middle && (*ref_x_and_o).middle_middle == (*ref_x_and_o).bottom_left && (*ref_x_and_o).bottom_left != ' '{
            (*ref_game_settings).game_won = true;
            (ref_game_settings).winner_symbol = (*ref_x_and_o).top_right;
            sleep(Duration::from_millis(750));
        }
    }
}

pub fn choose_next_player(ref_game_settings: &mut GameSettings){

    if (*ref_game_settings).players_turn == 'X'{
        (*ref_game_settings).players_turn = 'O';
    }

    else{
        (*ref_game_settings).players_turn = 'X';
    }
}

pub fn grab_character_from_player(ref_game_settings: &mut GameSettings, ref_x_and_o: &mut XAndOValues){
    let possible_characters: Vec<String> = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"]
                                                .iter()
                                                .map(|s| s.to_string())
                                                .collect();
    let mut user_input: String = String::new();
    let mut good_x_or_o_placement: bool = false;

    while !possible_characters.contains(&user_input){
        while !good_x_or_o_placement{
            user_input = String::new();

            print!("Current Placement ({}): ", (*ref_game_settings).players_turn);

            // Flush the output to ensure the prompt is displayed
            io::stdout().flush().expect("Failed to flush");

            io::stdin().read_line(&mut user_input).expect("Failed to read line");

            user_input = user_input.trim().to_string(); // Removing all weird characters i.e. '\r', '\n' and '\t'

            // Ensure that markers are not placing over older markers
            good_x_or_o_placement = place_character_in_position(ref_x_and_o, ref_game_settings, &user_input);
        }
    }
}

fn place_character_in_position(ref_x_and_o: &mut XAndOValues, ref_game_settings: &mut GameSettings, ref_x_or_o_placement: &String) -> bool{
    match (*ref_x_or_o_placement).as_str(){
        "1" =>{
            if (*ref_x_and_o).top_left == ' '{
                (*ref_x_and_o).top_left = (*ref_game_settings).players_turn;
                return true;
            } 
            else {return false;}
        },
        "2" => {
            if (*ref_x_and_o).top_middle == ' '{
                (*ref_x_and_o).top_middle = (*ref_game_settings).players_turn;
                return true;
            } 
            else {return false;}
        },
        "3" => {
            if (*ref_x_and_o).top_right == ' '{
                (*ref_x_and_o).top_right = (*ref_game_settings).players_turn;
                return true;
            } 
            else {return false;}
        },
        "4" => {
            if (*ref_x_and_o).middle_left == ' '{
                (*ref_x_and_o).middle_left = (*ref_game_settings).players_turn;
                return true;
            } 
            else {return false;}

        },
        "5" => {
            if (*ref_x_and_o).middle_middle == ' '{
                (*ref_x_and_o).middle_middle = (*ref_game_settings).players_turn;
                return true;
            } 
            else {return false;}
        },
        "6" => {
            if (*ref_x_and_o).middle_right == ' '{
                (*ref_x_and_o).middle_right = (*ref_game_settings).players_turn;
                return true;
            } 
            else {return false;}
        },
        "7" => {
            if (*ref_x_and_o).bottom_left == ' '{
                (*ref_x_and_o).bottom_left = (*ref_game_settings).players_turn;
                return true;
            } 
            else {return false;}
        },
        "8" => {
            if (*ref_x_and_o).bottom_middle == ' '{
                (*ref_x_and_o).bottom_middle = (*ref_game_settings).players_turn;
                return true;
            } 
            else {return false;}
        },
        "9" => {
            
            if (*ref_x_and_o).bottom_right == ' '{
                (*ref_x_and_o).bottom_right = (*ref_game_settings).players_turn;
                return true;
            } 
            else {return false;}
        },
        _ => {return false;}
    }
}

pub fn check_if_stalemate(ref_x_and_o: &XAndOValues, ref_game_settings: &mut GameSettings){
    if (*ref_x_and_o).top_left != ' ' && 
    (*ref_x_and_o).top_middle != ' ' && 
    (*ref_x_and_o).top_right != ' ' && 
    (*ref_x_and_o).middle_left != ' ' && 
    (*ref_x_and_o).middle_middle != ' ' && 
    (*ref_x_and_o).middle_right != ' ' && 
    (*ref_x_and_o).bottom_left != ' ' && 
    (*ref_x_and_o).bottom_middle != ' ' && 
    (*ref_x_and_o).bottom_right != ' ' {
        (*ref_game_settings).stalemate_achieved = true;
    }
}