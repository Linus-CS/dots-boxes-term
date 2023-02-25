const BOX_MASK: u16 = 0b0000_0000_1111_0000;

pub const PLAYER_ONE: u8 = 9;
pub const PLAYER_TWO: u8 = 8;

pub const LEFT: u8 = 3;
pub const TOP: u8 = 2;
pub const RIGHT: u8 = 1;
pub const BOTTOM: u8 = 0;

pub struct GameInfo {
    score: [u8; 2],
    turn: u8,
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

struct Board {
    layout: [u16; 100],
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

    fn get_bit(&self, row: usize, column: usize, shift: u8) -> bool {
        return self.layout[row * 10 + column] & (1 << shift) > 0;
    }
}

pub struct BorderPatrol {
    board: Board,
    game_info: GameInfo,
    horizontal_indices: Vec<usize>,
    vertical_indices: Vec<usize>,
    orientation: usize,
    pos: usize,
}

impl BorderPatrol {
    pub fn new() -> BorderPatrol {
        BorderPatrol {
            board: Board::new(),
            game_info: GameInfo::new(),
            horizontal_indices: vec![],
            vertical_indices: vec![],
            orientation: 0,
            pos: 0,
        }
    }

    pub fn set_line(&mut self, num: usize, player: u8) {
        let box_num = num / 4;
        let row = box_num / 10;
        let col = box_num % 10;
        let side = (num % 4).try_into().unwrap();

        self.set_line_by(row, col, side, player);
    }

    pub fn set_line_by(&mut self, row: usize, column: usize, side: u8, player: u8) {
        self.board.set_bit(row, column, side + (4 * (player % 2)));
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

pub mod display {
    /*
                top
        left            right       0b0000 0|0|p1|p2 lp1|tp1|rp1|bp1 lp2|tp2|rp2|bp2
                bottom

        right left for player one would be 0b0000 0000 0000 1010


                ██    ░░   ┌────┬    ┬────┐    └────┴    ┴────┘     ├────┼   ┼────┤

                ╣  ║  ╗  ╝  ╚  ╔  ╩ ╦ ╠ ═ ╬             ┣  ┫  ┛  ┳  ┻  ╋  ┃  ┗  ┏  ┓ ━
    */
    use super::{BorderPatrol, BOTTOM, PLAYER_ONE, PLAYER_TWO, RIGHT};
    use crate::engine::Game;

    const LINE_OFFSET: u8 = 4;
    const LINES: [&str; 12] = [
        "────",
        "│",
        "────",
        "│",
        "════",
        "║",
        "════",
        "║",
        "━━━━",
        "┃",
        "━━━━",
        "┃",
    ];
    const BOXES: [&str; 2] = ["██", "░░"];

    impl BorderPatrol {
        fn get_line_display(&self, row: usize, column: usize, side: u8) -> &str {
            let player_one_line = LINES[(4 + side + LINE_OFFSET) as usize];
            let player_two_line = LINES[(side + LINE_OFFSET) as usize];
            let line = LINES[side as usize];

            if self.board.get_bit(row, column, side + 4) {
                return player_one_line;
            }
            if self.board.get_bit(row, column, side) {
                return player_two_line;
            }

            line
        }

        fn get_box_display(&self, row: usize, column: usize) -> &str {
            if self.board.get_bit(row, column, PLAYER_ONE) {
                return BOXES[0];
            }
            if self.board.get_bit(row, column, PLAYER_TWO) {
                return BOXES[1];
            }

            "  "
        }
    }

    impl Game for BorderPatrol {
        fn init_screen(&mut self) -> String {
            let mut content: String = format!("\n\n\x1B[1m\t\t\t\t\t\t\t\t   Borderpatrol\n\n\t\t\t\t\t\t        PlayerOne     {} - {}      PlayerTwo\n",
            self.game_info.get_player_one_points(),
            self.game_info.get_player_two_points()
        );
            content += "\t\t\t\t\t\t┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐\n";
            let mut len = content.len();
            for r in 0..10 {
                let mut box_row: String = "\t\t\t\t\t\t│ ".to_owned();
                let mut line_row: String = "\t\t\t\t\t\t├".to_owned();
                for c in 0..10 {
                    box_row += self.get_box_display(r, c);
                    box_row += " ";
                    box_row += self.get_line_display(r, c, RIGHT);
                    box_row += " ";
                    line_row += self.get_line_display(r, c, BOTTOM);
                    line_row += "┼";
                }
                box_row.pop();
                box_row.pop();
                line_row.pop();
                content += &box_row;
                content += "│\n";
                len = content.len();
                content += &line_row;
                content += "┤\n";
            }
            content.truncate(len - 1);

            self.horizontal_indices = Vec::new();
            self.vertical_indices = Vec::new();

            content
                .match_indices(LINES[0])
                .for_each(|(index, _)| self.horizontal_indices.push(index));
            content
                .match_indices(LINES[4])
                .for_each(|(index, _)| self.horizontal_indices.push(index));
            content
                .match_indices(LINES[8])
                .for_each(|(index, _)| self.horizontal_indices.push(index));

            content
                .match_indices(LINES[5])
                .for_each(|(index, _)| self.vertical_indices.push(index));
            content
                .match_indices(LINES[9])
                .for_each(|(index, _)| self.vertical_indices.push(index));
            content
                .match_indices(LINES[1])
                .for_each(|(index, _)| self.vertical_indices.push(index));

            self.horizontal_indices.sort();
            self.horizontal_indices.splice(0..10, None);
            let mut cnt = -1;
            self.vertical_indices.sort();
            self.vertical_indices = self
                .vertical_indices
                .iter()
                .filter(|_| {
                    cnt += 1;
                    if cnt != 0 && (cnt - 1) % 11 < 9 {
                        return true;
                    }
                    return false;
                })
                .map(|a| *a)
                .collect();

            println!("h: {:?}", self.horizontal_indices);

            content +=
            "\n\t\t\t\t\t\t└────┴────┴────┴────┴────┴────┴────┴────┴────┴────┘\n\n\n\n\n\n\n\n\n";

            return content;
        }

        fn react(&mut self, content: &mut String, key: char) -> bool {
            match key {
                'h' if self.pos > 0 => {
                    self.pos -= 1;
                }
                'j' if self.pos < 80 => {
                    self.pos += 10 - self.orientation;
                }
                'k' if self.pos > 9 => {
                    self.pos -= 10 - self.orientation;
                }
                'l' if self.pos < 89 => {
                    self.pos += 1;
                }
                'f' => {
                    if self.orientation == 1 {
                        self.pos += self.pos / 9;
                        if self.pos > 89 {
                            self.pos -= 10;
                        }
                    } else {
                        self.pos -= self.pos / 10;
                    }
                    self.orientation = 1 - self.orientation
                }
                _ => (),
            };

            let index = if self.orientation == 0 {
                self.horizontal_indices[self.pos]
            } else {
                self.vertical_indices[self.pos]
            };

            if key == ' ' {
                let correct_pos = if self.orientation == 1 {
                    self.pos + self.pos / 9
                } else {
                    self.pos
                };

                self.set_line_by(
                    correct_pos / 10,
                    correct_pos % 10,
                    self.orientation as u8,
                    self.game_info.turn,
                );
                content.replace_range(0..content.len(), &self.init_screen());
                return true;
            }

            if self.orientation == 0 {
                content.replace_range((index + 3)..(index + 9), "○○");
            } else {
                content.replace_range(index..(index + 3), "○");
            }
            return false;
        }
    }
}

pub mod machine_learning {

    use rand::seq::IteratorRandom;

    use super::{BorderPatrol, PLAYER_ONE, PLAYER_TWO};
    use crate::ml::Environment;

    impl Environment<[u16; 100]> for BorderPatrol {
        fn step(&mut self, action: usize) -> ([u16; 100], f64, bool) {
            let player = self.game_info.turn;

            let before = self.game_info.get_points();
            self.set_line(action, self.game_info.turn);
            let after = self.game_info.score[(PLAYER_ONE - player) as usize];

            let mut reward = (before - after) as f64 * 0.05;
            let mut done = false;

            if after > 50 {
                done = true;
                reward += 1.0;
            }

            if after == 50 && self.game_info.get_points() == 50 {
                done = true;
            }
            return (self.get_state(), reward, done);
        }

        fn get_state(&self) -> [u16; 100] {
            self.board.layout
        }

        fn random_action(&self) -> usize {
            let (box_num, value) = self
                .board
                .layout
                .iter()
                .enumerate()
                .filter(|&(_, x)| (x >> PLAYER_TWO) == 0)
                .choose(&mut rand::thread_rng())
                .unwrap();

            let mut side = 0;
            let offset = 4 * (self.game_info.turn % 2);
            loop {
                if (value >> offset) & (1 << side) == 0 {
                    break;
                }
                side += 1;
            }

            box_num * 4 + side
        }
    }
}
