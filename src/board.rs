#[derive(PartialEq)]
pub enum Team {
    X,
    O,
}

impl Clone for Team {
    fn clone(&self) -> Team {
        match self {
            Self::X => Team::X,
            Self::O => Team::O,
        }
    }
}

pub struct BoardPosition {
    pub x: u8,
    pub y: u8,
}

impl Clone for BoardPosition {
    fn clone(&self) -> BoardPosition {
        BoardPosition {
            x: self.x,
            y: self.y,
        }
    }
}

pub struct Board {
    // 2 bits per cell. First bit is the value, second is the team. X=0 O=1
    data: u32,
    pub current_player: Team,
}

impl Clone for Board {
    fn clone(&self) -> Board {
        Board {
            data: self.data,
            current_player: self.current_player.clone(),
        }
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            data: 0,
            current_player: Team::X,
        }
    }

    #[inline(always)]
    fn get_is_occupied_mask(pos: &BoardPosition) -> u32 {
        1 << ((pos.y as u32 * 3 + pos.x as u32) * 2)
    }

    #[inline(always)]
    fn get_team_state_mask(pos: &BoardPosition) -> u32 {
        1 << (((pos.y as u32 * 3 + pos.x as u32) * 2) + 1)
    }

    // Returns true if the current player won
    pub fn evaluate(&self) -> bool {
        // Check -
        let mut y = 0;
        while y < 3 {
            let full_row_mask = 0b111111 << (y * 6);
            let comp_mask = match self.current_player {
                Team::X => 0b010101 << (y * 6),
                Team::O => 0b111111 << (y * 6),
            };

            if (self.data & full_row_mask) == comp_mask {
                return true;
            }

            y = y + 1;
        }

        // Check |
        let mut x = 0;
        while x < 3 {
            let full_col_mask: u32 = 0b11000011000011 << (x * 2);
            let comp_mask = match self.current_player {
                Team::X => 0b01000001000001 << (x * 2),
                Team::O => 0b11000011000011 << (x * 2),
            };

            if (self.data & full_col_mask) == comp_mask {
                return true;
            }

            x = x + 1;
        }

        // Check \
        {
            let full_did_mask = 0b110000001100000011;
            let comp_mask = match self.current_player {
                Team::X => 0b010000000100000001,
                Team::O => 0b110000001100000011,
            };

            if (self.data & full_did_mask) == comp_mask {
                return true;
            }
        }

        // Check /
        {
            let full_diu_mask = 0b11001100110000;
            let comp_mask = match self.current_player {
                Team::X => 0b01000100010000,
                Team::O => 0b11001100110000,
            };

            if (self.data & full_diu_mask) == comp_mask {
                return true;
            }
        }

        return false;
    }

    pub fn is_full(&self) -> bool {
        (self.data & 0b010101010101010101) == 0b010101010101010101
    }

    pub fn is_cell_empty(&self, pos: BoardPosition) -> bool {
        (self.data & Board::get_is_occupied_mask(&pos)) == 0
    }

    pub fn get_cell(&self, pos: BoardPosition) -> Option<Team> {
        if !self.is_cell_empty(pos.clone()) {
            if (self.data & Board::get_team_state_mask(&pos)) != 0 {
                return Some(Team::O);
            } else {
                return Some(Team::X);
            }
        }
        None
    }

    pub fn set_cell(&mut self, pos: BoardPosition) {
        self.data |= Board::get_is_occupied_mask(&pos);
        if self.current_player == Team::O {
            self.data |= Board::get_team_state_mask(&pos);
        }
    }

    pub fn swap_players(&mut self) {
        if self.current_player == Team::X {
            self.current_player = Team::O;
        } else {
            self.current_player = Team::X;
        }
    }
}
