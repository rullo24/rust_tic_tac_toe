use crate::data_structures::{XAndOValues, GameSettings};

pub fn print_map(ref_x_and_o: &XAndOValues){
    clear_terminal();

    // Printing Game to Terminal
    println!("*************");
    println!("* {} | {} | {} *       | 1 | 2 | 3 |", 
            (*ref_x_and_o).top_left,
            (*ref_x_and_o).top_middle,
            (*ref_x_and_o).top_right);

    println!("*-----------*");

    println!("* {} | {} | {} *       | 4 | 5 | 6 |", 
            (*ref_x_and_o).middle_left,
            (*ref_x_and_o).middle_middle,
            (*ref_x_and_o).middle_right);

    println!("*-----------*");
    
    println!("* {} | {} | {} *       | 7 | 8 | 9 |", 
            (*ref_x_and_o).bottom_left,
            (*ref_x_and_o).bottom_middle,
            (*ref_x_and_o).bottom_right);
    println!("*************");
}

fn clear_terminal() {
        // Specific command that is used by the print macro to clear the terminal screen.
        print!("\x1B[2J\x1B[1;1H");
}

pub fn terminal_print_winner(ref_game_settings: &GameSettings){

        let winning_char = (*ref_game_settings).winner_symbol.clone();

        clear_terminal();

        if (*ref_game_settings).stalemate_achieved{
                println!("--------------------------------------------\n");

                println!("\t### STALEMATE ###");

                println!("\n--------------------------------------------");  
        }
        else{
                println!("--------------------------------------------\n");

                println!("\t### {} WINS ###", winning_char);

                println!("\n--------------------------------------------");
        }
        
}