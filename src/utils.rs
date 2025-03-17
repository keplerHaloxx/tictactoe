use std::{
    io::{Write, stdin, stdout},
    process::Command,
};

use crate::board::Player;

pub fn get_input(message: &str) -> String {
    print!("{}", message);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub fn get_move_number(player: Player) -> usize {
    loop {
        if let Ok(num) =
            get_input(&format!("Enter your move ({}) (1-9): ", player)).parse::<usize>()
        {
            if is_valid_num(num) {
                return num;
            }
        }
        println!("Please choose a number 1 through 9!\n");
    }
}

/// Checks if a number is 1-9 (for tictactoe ofc)
pub fn is_valid_num(num: usize) -> bool {
    num >= 1 && num <= 9
}

pub fn clear_screen() {
    if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        Command::new("clear").status().unwrap();
    } else if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    }
}
