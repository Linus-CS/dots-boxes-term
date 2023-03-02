pub mod display {
    /*
                top
        left            right       0b0000 0|0|p1|p2 lp1|tp1|rp1|bp1 lp2|tp2|rp2|bp2
                bottom

        right left for player one would be 0b0000 0000 0000 1010


                ██    ░░   ┌────┬    ┬────┐    └────┴    ┴────┘     ├────┼   ┼────┤

                ╣  ║  ╗  ╝  ╚  ╔  ╩ ╦ ╠ ═ ╬             ┣  ┫  ┛  ┳  ┻  ╋  ┃  ┗  ┏  ┓ ━
    */
    use crate::{
        borderpatrol::{BorderPatrol, BOTTOM, PLAYER_ONE, PLAYER_TWO, RIGHT},
        engine::Game,
    };

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
    pub trait Player {
        fn init(&self);
        fn make_move(&self, border_patrol: &mut BorderPatrol);
    }

    pub struct TerminalBorderPatrol {
        pub border_patrol: BorderPatrol,
        pub horizontal_indices: Vec<usize>,
        pub vertical_indices: Vec<usize>,
        pub orientation: usize,
        pub pos: usize,
        player_one: Option<Box<dyn Player>>,
        player_two: Option<Box<dyn Player>>,
    }

    impl TerminalBorderPatrol {
        pub fn new() -> Self {
            TerminalBorderPatrol {
                border_patrol: BorderPatrol::new(),
                horizontal_indices: vec![],
                vertical_indices: vec![],
                orientation: 0,
                pos: 0,
                player_one: None,
                player_two: None,
            }
        }

        pub fn with_players(player_one: Box<dyn Player>, player_two: Box<dyn Player>) -> Self {
            let mut instance = Self::new();
            instance.player_one = Some(player_one);
            instance.player_two = Some(player_two);
            instance
        }

        pub fn with_player_one(player: Box<dyn Player>) -> Self {
            let mut instance = Self::new();
            instance.player_one = Some(player);
            instance
        }

        pub fn with_player_two(player: Box<dyn Player>) -> Self {
            let mut instance = Self::new();
            instance.player_two = Some(player);
            instance
        }
    }

    impl TerminalBorderPatrol {
        fn get_line_display(&self, row: usize, column: usize, side: u8) -> &str {
            let player_one_line = LINES[(4 + side + LINE_OFFSET) as usize];
            let player_two_line = LINES[(side + LINE_OFFSET) as usize];
            let line = LINES[side as usize];

            if self.border_patrol.board.get_bit(row, column, side + 4) {
                return player_one_line;
            }
            if self.border_patrol.board.get_bit(row, column, side) {
                return player_two_line;
            }

            line
        }

        fn get_box_display(&self, row: usize, column: usize) -> &str {
            if self.border_patrol.board.get_bit(row, column, PLAYER_ONE) {
                return BOXES[0];
            }
            if self.border_patrol.board.get_bit(row, column, PLAYER_TWO) {
                return BOXES[1];
            }

            "  "
        }

        fn determine_indices(&mut self) {
            let content = self.to_string();

            self.horizontal_indices = Vec::new();
            self.vertical_indices = Vec::new();

            for (i, j) in vec![0, 4, 8].into_iter().zip(vec![5, 9, 1]) {
                content
                    .match_indices(LINES[i])
                    .for_each(|(index, _)| self.horizontal_indices.push(index));
                content
                    .match_indices(LINES[j])
                    .for_each(|(index, _)| self.vertical_indices.push(index));
            }

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
        }

        fn to_string(&self) -> String {
            let mut content: String = format!(
                "\n\n\x1B[1m{}   Borderpatrol\n\n{}        PlayerOne     {} - {}      PlayerTwo\n",
                "\t".repeat(8),
                "\t".repeat(6),
                self.border_patrol.game_info.get_player_one_points(),
                self.border_patrol.game_info.get_player_two_points()
            );

            content.push_str(&("\t".repeat(6) + "┌" + &"────┬".repeat(9) + "────┐\n"));
            let mut len = content.len();
            for r in 0..10 {
                let mut box_row: String = "\t".repeat(6) + "│ ";
                let mut line_row: String = "\t".repeat(6) + "├";
                for c in 0..10 {
                    box_row.push_str(&(self.get_box_display(r, c).to_owned() + " "));
                    box_row.push_str(&(self.get_line_display(r, c, RIGHT).to_owned() + " "));
                    line_row.push_str(&(self.get_line_display(r, c, BOTTOM).to_owned() + "┼"));
                }
                box_row.pop();
                box_row.pop();
                line_row.pop();
                content.push_str(&(box_row + "│\n"));
                len = content.len();
                content.push_str(&(line_row + "┤\n"));
            }
            content.truncate(len - 1);

            content.push_str(
                &("\n".to_owned()
                    + &"\t".repeat(6)
                    + "└"
                    + &"────┴".repeat(9)
                    + "────┘"
                    + &"\n".repeat(9)),
            );

            content
        }
    }

    impl Game for TerminalBorderPatrol {
        fn update(&mut self, content: &mut String) {
            if self.border_patrol.game_info.turn == PLAYER_ONE {
                if let Some(player) = &self.player_one {
                    player.make_move(&mut self.border_patrol);
                }
                *content = self.to_string();
                return;
            }

            if let Some(player) = &self.player_two {
                player.make_move(&mut self.border_patrol);
            }
            *content = self.to_string();
        }

        fn wait_for_input(&self) -> bool {
            if self.player_one.is_some() && self.border_patrol.game_info.turn == PLAYER_ONE {
                return false;
            }

            if self.player_two.is_some() && self.border_patrol.game_info.turn == PLAYER_TWO {
                return false;
            }

            true
        }

        fn init_screen(&mut self) -> String {
            self.determine_indices();
            self.to_string()
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

                self.border_patrol.set_line_by(
                    correct_pos / 10,
                    correct_pos % 10,
                    self.orientation as u8,
                    self.border_patrol.game_info.turn,
                );
                content.replace_range(0..content.len(), &self.init_screen());
                return true;
            }

            if self.orientation == 0 {
                content.replace_range((index + 3)..(index + 9), "○○");
            } else {
                content.replace_range(index..(index + 3), "○");
            }
            false
        }
    }
}

pub mod machine_learning {

    use rand::seq::IteratorRandom;

    use crate::{
        borderpatrol::{BorderPatrol, PLAYER_ONE, PLAYER_TWO},
        ml::Environment,
    };

    impl Environment<[u16; 100]> for BorderPatrol {
        fn step(&mut self, action: usize) -> f64 {
            let player = self.game_info.turn;

            let before = self.game_info.get_points();
            self.set_line(action, self.game_info.turn);
            let after = self.game_info.score[(PLAYER_ONE - player) as usize];

            let mut reward = (after - before) as f64 * 0.05;

            if after > 50 {
                self.game_info.finished = true;
                reward += 1.0;
            }

            if after == 50 && self.game_info.get_points() == 50 {
                self.game_info.finished = true;
            }
            return reward;
        }

        fn get_turn(&self) -> usize {
            (PLAYER_ONE - self.game_info.turn) as usize
        }

        fn is_finished(&self) -> bool {
            return self.game_info.finished;
        }

        fn get_state(&self) -> [u16; 100] {
            self.board.layout
        }

        fn get_possible_actions(&self) -> &[bool] {
            return self.possible_actions.as_slice();
        }

        fn random_action(&self) -> usize {
            self.possible_actions
                .iter()
                .enumerate()
                .filter(|&(_, x)| *x)
                .choose(&mut rand::thread_rng())
                .unwrap()
                .0
        }
    }
}

pub mod players {
    use crate::{
        borderpatrol::BorderPatrol,
        ml::{AgentNN, Environment},
    };

    use super::display::Player;

    impl<F, E> Player for AgentNN<F, E>
    where
        F: Fn() -> Box<dyn Environment<E>>,
    {
        fn init(&self) {}

        fn make_move(&self, border_patrol: &mut BorderPatrol) {
            let converted_state: Vec<f64> = border_patrol
                .board
                .layout
                .clone()
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect();

            let possible = &border_patrol.possible_actions;

            let action = self
                .nn
                .run(&converted_state)
                .into_iter()
                .enumerate()
                .filter(|&(i, _)| possible[i])
                .reduce(|acc, x| if acc.1 >= x.1 { acc } else { x })
                .unwrap()
                .0;

            border_patrol.set_line(action, border_patrol.game_info.turn);
        }
    }
}
