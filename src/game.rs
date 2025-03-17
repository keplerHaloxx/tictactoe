use crate::{
    board::{Board, Cell, Player},
    utils::{clear_screen, get_move_number, is_valid_num},
};

#[derive(Debug)]
pub enum WinType {
    X,
    O,
    Draw,
    Ongoing,
}

#[derive(Debug)]
pub struct RoundResult {
    pub player: Player,
    pub result: WinType,
}

pub struct Game {
    pub board: Board,
    pub current_player_turn: Player,
}

impl Game {
    pub fn new(starting_player: Player) -> Self {
        Self {
            board: Board::default(),
            current_player_turn: starting_player,
        }
    }

    pub fn start(&mut self) {
        loop {
            // clear and show current state
            clear_screen();
            println!("{}", self.board);

            let round = self.next_move();

            // clear and show new state
            clear_screen();
            println!("{}", self.board);

            match round.result {
                WinType::X | WinType::O => {
                    println!("Congratulations {}, you have won!", round.player);
                    break;
                }
                WinType::Draw => {
                    println!("It's a draw.");
                    break;
                }
                WinType::Ongoing => continue,
            }
        }
    }

    pub fn is_draw(&self) -> bool {
        let is_board_filled = self.board.cells.iter().all(|c| {
            if let Cell::Occupied(_) = c {
                return true;
            }
            false
        });

        is_board_filled && self.board.check_winner().is_none()
    }

    pub fn next_move(&mut self) -> RoundResult {
        let mut move_num: usize;
        // loop till we get a valid number
        loop {
            move_num = get_move_number(self.current_player_turn);
            if !is_valid_num(move_num) {
                println!("Please choose a number 1 through 9!\n");
                continue;
            }
            if !self.board.is_cell_vacant(move_num).unwrap() {
                println!("Please choose a free cell!\n");
                continue;
            }

            break;
        }

        // just a precaution
        let current_player = self.current_player_turn;

        self.board
            .set_cell(move_num, Cell::Occupied(current_player))
            .unwrap();

        let mut result: WinType = WinType::Ongoing;
        if self.is_draw() {
            result = WinType::Draw;
        }
        if let Some(player) = self.board.check_winner() {
            if player == current_player {
                result = current_player.into();
            } else {
                result = current_player.flip().into();
            }
        }

        self.switch_current_player();

        RoundResult {
            player: current_player,
            result,
        }
    }

    pub fn switch_current_player(&mut self) {
        match self.current_player_turn {
            Player::X => self.current_player_turn = Player::O,
            Player::O => self.current_player_turn = Player::X,
        }
    }
}
