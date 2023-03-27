#![allow(dead_code)]

use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use nn::{HaltCondition, NN};
use rand::{seq::IteratorRandom, Rng};

use crate::{
    borderpatrol::{BorderPatrol, PLAYER_ONE, PLAYER_TWO},
    terminal_borderpatrol::display::Player,
};

pub struct HyperParameters {
    pub discount: f64,
    pub eps: f64,
    pub eps_decay: f64,
    pub episodes: usize,
}

pub struct BorderPatrolAgent {
    pub hyper_parms: HyperParameters,
    pub nn: NN,
}

impl BorderPatrolAgent {
    pub fn new(shape: &[u32]) -> BorderPatrolAgent {
        BorderPatrolAgent {
            hyper_parms: HyperParameters {
                discount: 0.95,
                eps: 0.5,
                eps_decay: 0.999,
                episodes: 500,
            },
            nn: NN::new(shape),
        }
    }

    pub fn from_json(json: String) -> BorderPatrolAgent {
        BorderPatrolAgent {
            hyper_parms: HyperParameters {
                discount: 0.95,
                eps: 0.5,
                eps_decay: 0.999,
                episodes: 500,
            },
            nn: NN::from_json(&json),
        }
    }

    pub fn new_with(shape: &[u32], hyper_parms: HyperParameters) -> BorderPatrolAgent {
        BorderPatrolAgent {
            hyper_parms,
            nn: NN::new(shape),
        }
    }

    pub fn from_file(file_path: &str) -> BorderPatrolAgent {
        let path = Path::new(file_path);
        let mut file = match File::open(&path) {
            Ok(file) => file,
            _ => panic!("Could not open file at {file_path}"),
        };
        let mut json = String::new();
        match file.read_to_string(&mut json) {
            Ok(_) => println!("Read from {file_path}."),
            _ => panic!("Could not read from {file_path}"),
        }
        return Self::from_json(json);
    }

    pub fn save(&self, file_path: &str) {
        let path = Path::new(file_path);
        let mut file = match File::create(&path) {
            Ok(file) => file,
            _ => panic!("Could not create file at {file_path}"),
        };
        match file.write_all(self.nn.to_json().as_bytes()) {
            Ok(_) => println!("Wrote to {file_path}."),
            _ => panic!("Could not write to {file_path}"),
        };
    }

    fn get_action(&self, r: f64, env: &BorderPatrol, state: &Vec<f64>) -> usize {
        if r < self.hyper_parms.eps {
            env.random_action()
        } else {
            self.nn
                .run(state)
                .into_iter()
                .enumerate()
                .filter(|&(i, _)| env.is_possible(i))
                .reduce(|acc, x| if acc.1 >= x.1 { acc } else { x })
                .unwrap()
                .0
        }
    }

    fn convert_state(&self, state: [u16; 100]) -> Vec<f64> {
        state.into_iter().map(|x| x.try_into().unwrap()).collect()
    }

    pub fn train(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.hyper_parms.episodes {
            println!("Episode {i}");
            let mut env = BorderPatrol::new();

            let mut states = Vec::with_capacity(2);
            let mut actions: [usize; 2] = [0, 0];
            let mut rewards: [f64; 2] = [0.0, 0.0];

            states.push(self.convert_state(env.board.layout));
            actions[0] = self.get_action(rng.gen_range(0.0..1.0), &env, &states[0]);
            rewards[0] = env.step(actions[0]);

            states.push(self.convert_state(env.board.layout));
            actions[1] = self.get_action(rng.gen_range(0.0..1.0), &env, &states[1]);
            rewards[1] = env.step(actions[1]);

            let mut state = self.convert_state(env.board.layout);

            let mut turn = env.get_turn();

            self.hyper_parms.eps *= self.hyper_parms.eps_decay;

            while !env.game_info.finished {
                // Get predicted move form neural network for the new state
                let target = rewards[turn]
                    + self.hyper_parms.discount
                        * self
                            .nn
                            .run(&state)
                            .into_iter()
                            .enumerate()
                            .filter(|&(i, _)| env.is_possible(i))
                            .map(|(_, x)| x)
                            .reduce(f64::max)
                            .unwrap();

                // Adjust values to account for target value of new state
                let mut target_vec = self.nn.run(&states[turn]);
                target_vec[actions[turn]] = target;

                actions[turn] = self.get_action(rng.gen_range(0.0..1.0), &env, &state);
                rewards[turn] = env.step(actions[turn]);
                rewards[(turn + 1) % 2] -= rewards[turn] / 10.0;

                // Train neural network on the target value with the current state
                if turn == 0 {
                    self.nn
                        .train(&[(state, target_vec)])
                        .halt_condition(HaltCondition::Epochs(1))
                        .go();
                }

                state = self.convert_state(env.board.layout);
                turn = env.get_turn();
            }
        }
    }
}

impl BorderPatrol {
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

    fn is_possible(&self, line: usize) -> bool {
        !(self.get_line(line, PLAYER_ONE) || self.get_line(line, PLAYER_TWO))
    }

    fn get_turn(&self) -> usize {
        (PLAYER_ONE - self.game_info.turn) as usize
    }

    fn random_action(&self) -> usize {
        let offset = self.game_info.turn - PLAYER_TWO;
        let (i, value) = self
            .board
            .layout
            .iter()
            .enumerate()
            .filter(|&(_, x)| (x >> offset) & 3 != 3)
            .choose(&mut rand::thread_rng())
            .unwrap();

        if (value >> offset) & 1 == 0 {
            return i;
        }

        i + 1
    }
}

impl Player for BorderPatrolAgent {
    fn init(&mut self) {}

    fn make_move(&self, border_patrol: &mut BorderPatrol) {
        let converted_state: Vec<f64> = border_patrol
            .board
            .layout
            .clone()
            .into_iter()
            .map(|x| x.try_into().unwrap())
            .collect();

        let action = self
            .nn
            .run(&converted_state)
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| {
                !(border_patrol.get_line(i, PLAYER_ONE) || border_patrol.get_line(i, PLAYER_TWO))
            })
            .reduce(|acc, x| if acc.1 >= x.1 { acc } else { x })
            .unwrap()
            .0;

        border_patrol.set_line(action, border_patrol.game_info.turn);
    }
}
