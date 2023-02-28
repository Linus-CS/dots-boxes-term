#![allow(dead_code)]
const BOX_MASK: u16 = 0b0000_0000_1111_0000;

pub const PLAYER_ONE: u8 = 9;
pub const PLAYER_TWO: u8 = 8;

pub const LEFT: u8 = 3;
pub const TOP: u8 = 2;
pub const RIGHT: u8 = 1;
pub const BOTTOM: u8 = 0;

pub struct GameInfo {
    pub score: [u8; 2],
    pub turn: u8,
}

impl GameInfo {
    pub fn new() -> GameInfo {
        return GameInfo {
            score: [0, 0],
            turn: PLAYER_ONE,
        };
    }

    pub fn get_points(&self) -> u8 {
        self.score[(PLAYER_ONE - self.turn) as usize]
    }

    pub fn get_player_one_points(&self) -> u8 {
        self.score[0]
    }

    pub fn get_player_two_points(&self) -> u8 {
        self.score[1]
    }
}

pub struct Board {
    pub layout: [u16; 100],
}

impl Board {
    fn new() -> Board {
        Board {
            layout: [
                204, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 0, 0, 0, 0, 0, 0, 0, 0, 34, 136, 0, 0,
                0, 0, 0, 0, 0, 0, 34, 136, 0, 0, 0, 0, 0, 0, 0, 0, 34, 136, 0, 0, 0, 0, 0, 0, 0, 0,
                34, 136, 0, 0, 0, 0, 0, 0, 0, 0, 34, 136, 0, 0, 0, 0, 0, 0, 0, 0, 34, 136, 0, 0, 0,
                0, 0, 0, 0, 0, 34, 136, 0, 0, 0, 0, 0, 0, 0, 0, 34, 153, 17, 17, 17, 17, 17, 17,
                17, 17, 51,
            ],
        }
    }

    fn set_bit(&mut self, row: usize, column: usize, shift: u8) {
        self.layout[row * 10 + column] = self.layout[row * 10 + column] | (1 << shift);
    }

    pub fn get_bit(&self, row: usize, column: usize, shift: u8) -> bool {
        return self.layout[row * 10 + column] & (1 << shift) > 0;
    }
}

pub struct BorderPatrol {
    pub board: Board,
    pub game_info: GameInfo,
    pub possible_actions: [bool; 180],
}

impl BorderPatrol {
    pub fn new() -> BorderPatrol {
        BorderPatrol {
            board: Board::new(),
            game_info: GameInfo::new(),
            possible_actions: [true; 180],
        }
    }

    pub fn set_line(&mut self, num: usize, player: u8) {
        let box_num = num / 2;
        let row = box_num / 10;
        let col = box_num % 10;
        let side = (num % 2).try_into().unwrap();

        self.set_line_by(row, col, side, player);
    }

    pub fn set_line_by(&mut self, row: usize, column: usize, side: u8, player: u8) {
        self.board.set_bit(row, column, side + (4 * (player % 2)));
        self.possible_actions[row * 10 + column + side as usize] = false;
        self.check_box(row, column, player);

        let horizontal = (1 - side % 2) as i8;
        let h_amount = -1 * side as i8 + 1;
        let vertical = (side % 2) as i8;
        let v_amount = -1 * side as i8 + 2;

        let extra_row = (row as i8 + (horizontal * h_amount)) as usize;
        let extra_column = (column as i8 + (vertical * v_amount)) as usize;

        let extra_side =
            (side as i8 + (horizontal * (2 * h_amount) + (vertical * (2 * v_amount)))) as u8;

        self.board
            .set_bit(extra_row, extra_column, extra_side + (4 * (player % 2)));
        self.check_box(extra_row, extra_column, player);
        self.game_info.turn = 9 - (self.game_info.turn % PLAYER_TWO);
    }

    fn check_box(&mut self, row: usize, column: usize, player: u8) {
        if (self.board.layout[10 * row + column] & BOX_MASK)
            | (self.board.layout[10 * row + column] << 4 & BOX_MASK)
            == BOX_MASK
        {
            self.board.set_bit(row, column, player);
            self.game_info.score[(PLAYER_ONE - player) as usize] += 1;
            self.game_info.turn = 9 - (player % PLAYER_TWO);
        }
    }
}
