mod print_map;
mod data_structures;
mod check;

use print_map::*;
use data_structures::*;
use check::*;

fn main() {

    // Set all positions to empty strings originally
    let mut x_and_o: XAndOValues = XAndOValues::default();
    let mut game_settings: GameSettings = GameSettings::default();

    println!("{}", game_settings.game_won.clone());

    while game_settings.game_won == false{

        if game_settings.stalemate_achieved == true{
            break;
        }

        // Chooses the next player with every iteration | Doesn't make a difference for first iterations as this is randomised.
        choose_next_player(&mut game_settings);

        // Clear current terminal and print the naughts and crosses map.
        print_map(&x_and_o);

        // Ask user for placement of next character --> Repeat until character is acceptable.
        grab_character_from_player(&mut game_settings, &mut x_and_o);

        // Check if anyone has met requirements to finish game.
        check_if_won(&x_and_o, &mut game_settings);

        println!("{}", game_settings.game_won);

        // Check if all squares have been filled.
        check_if_stalemate(&x_and_o, &mut game_settings);
    }

    // Clearing the terminal and printing the winning char
    terminal_print_winner(&game_settings);
   
}
