use crate::{game::WinType, utils::is_valid_num};
use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

impl Into<WinType> for Player {
    fn into(self) -> WinType {
        match self {
            Player::X => WinType::X,
            Player::O => WinType::O,
        }
    }
}

impl Player {
    pub fn flip(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Occupied(Player),
    Vacant,
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Occupied(player) => write!(f, "{}", player),
            Cell::Vacant => write!(f, " "),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub cells: [Cell; 9],
}

impl Board {
    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &Cell>> {
        self.cells.chunks(3).map(|e| e.iter())
    }

    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &Cell>> {
        (0..3).map(move |n| self.cells.iter().skip(n).step_by(3))
    }

    pub fn get_cell(&self, cell: usize) -> Result<Cell, String> {
        if !is_valid_num(cell) {
            return Err("Index out of bounds, choose a number between 1-9".into());
        }

        Ok(self.cells[cell - 1])
    }

    pub fn set_cell(&mut self, cell: usize, new: Cell) -> Result<(), String> {
        if !is_valid_num(cell) {
            return Err("Index out of bounds, choose a number between 1-9".into());
        }

        self.cells[cell - 1] = new;

        Ok(())
    }

    pub fn is_cell_vacant(&self, cell: usize) -> Result<bool, String> {
        Ok(match self.get_cell(cell)? {
            Cell::Vacant => true,
            Cell::Occupied(_) => false,
        })
    }

    pub fn check_winner(&self) -> Option<Player> {
        for &player in &[Player::X, Player::O] {
            if self.has_winning_row(player)
                || self.has_winning_column(player)
                || self.has_winning_diagonal(player)
            {
                return Some(player);
            }
        }
        None
    }

    fn has_winning_row(&self, player: Player) -> bool {
        self.rows()
            .any(|mut row| row.all(|&cell| cell == Cell::Occupied(player)))
    }

    fn has_winning_column(&self, player: Player) -> bool {
        self.columns()
            .any(|mut col| col.all(|&cell| cell == Cell::Occupied(player)))
    }

    fn has_winning_diagonal(&self, player: Player) -> bool {
        let main_diag = (0..3).all(|i| self.cells[i * 3 + i] == Cell::Occupied(player));
        let anti_diag = (0..3).all(|i| self.cells[(i + 1) * 2] == Cell::Occupied(player));
        main_diag || anti_diag
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [Cell::Vacant; 9],
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+{}+", ["---"; 3].join("+"))?;
        for (i, row) in self.rows().enumerate() {
            writeln!(
                f,
                "| {} |",
                row.enumerate()
                    .map(|(j, c)| {
                        if let Cell::Vacant = c {
                            // turns the row and column (0 indexed) into a single number eg. (2,1) = 8
                            let current_pos = i * 3 + j + 1;
                            return format!("\x1b[2m{}\x1b[0m", current_pos); // dim the color
                        }
                        c.to_string()
                    })
                    .collect::<Vec<_>>()
                    .join(" | ")
            )?;
            writeln!(f, "+{}+", ["---"; 3].join("+"))?;
        }

        Ok(())
    }
}
