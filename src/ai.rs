use std::rc::Rc;

use crate::terminal_borderpatrol::display::Player;

#[derive(Debug)]
pub struct Line {
    value: usize,
}

#[derive(Debug)]
pub struct Square {
    value: usize,
    lines: [Option<Rc<Line>>; 4],
}

pub struct AdvancedPlayer {
    pub all_squares: Vec<Square>,
    pub all_lines: Vec<Rc<Line>>,
}

impl AdvancedPlayer {
    pub fn new() -> Self {
        let mut all_squares = Vec::with_capacity(100);
        let mut all_lines = Vec::with_capacity(200);

        for i in 0..100 {
            let row = i / 10;
            let col = i % 10;

            let mut value = 4;

            if row == 0 || row == 9 {
                value -= 1;
            }

            if col == 0 || col == 9 {
                value -= 1;
            }

            all_squares.push(Square {
                value,
                lines: [None, None, None, None],
            });
        }

        for i in 0..4 {
            all_lines.push(Rc::new(Line {
                value: all_squares[0].value,
            }));
            all_squares[0].lines[i] = Some(all_lines[i].clone());
        }

        let mut instance = AdvancedPlayer {
            all_squares,
            all_lines,
        };

        instance.insert_lines(1, 0, 0);
        instance
    }

    fn insert_lines(&mut self, square_id: usize, from_id: usize, side: usize) -> &mut Self {
        self.all_squares[square_id].lines[side] =
            self.all_squares[from_id].lines[((side + 2) % 4)].clone();

        let row = square_id / 10;
        let col = square_id % 10;

        for i in 0..4 {
            if self.all_squares[square_id].lines[i].is_none() {
                self.all_lines.push(Rc::new(Line {
                    value: self.all_squares[square_id].value,
                }));
                self.all_squares[square_id].lines[i] = Some(self.all_lines.last().unwrap().clone());

                if i == 0 && col > 0 {
                    self.insert_lines(square_id - 1, square_id, 2);
                }
                if i == 1 && row > 0 {
                    self.insert_lines(square_id - 10, square_id, 3);
                }
                if i == 2 && col < 9 {
                    self.insert_lines(square_id + 1, square_id, 0);
                }
                if i == 3 && row < 9 {
                    self.insert_lines(square_id + 10, square_id, 1);
                }
            }
        }

        self
    }
}

impl Player for AdvancedPlayer {
    fn init(&mut self) {}

    fn make_move(&self, border_patrol: &mut crate::borderpatrol::BorderPatrol) {
        todo!()
    }
}
