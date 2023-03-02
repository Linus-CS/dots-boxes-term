use nn::{HaltCondition, NN};
use rand::Rng;

use crate::borderpatrol::BorderPatrol;

pub struct HyperParameters {
    discount: f64,
    eps: f64,
    eps_decay: f64,
    episodes: usize,
}
pub trait Trainable {
    fn train(&mut self);
}

pub trait Environment<T> {
    fn step(&mut self, action: usize) -> f64;
    fn get_turn(&self) -> usize;
    fn get_state(&self) -> T;
    fn is_finished(&self) -> bool;
    fn random_action(&self) -> usize;
    fn get_possible_actions(&self) -> &[bool];
}

pub struct AgentNN<F, E>
where
    F: Fn() -> Box<dyn Environment<E>>,
{
    hyper_parms: HyperParameters,
    pub nn: NN,
    create_env: F,
}

impl<C, F, E> AgentNN<F, E>
where
    C: Into<f64>,
    F: Fn() -> Box<dyn Environment<E>>,
    E: IntoIterator<Item = C> + Clone,
{
    pub fn new(env_closure: F, shape: &[u32]) -> AgentNN<F, E> {
        AgentNN {
            hyper_parms: HyperParameters {
                discount: 0.95,
                eps: 0.5,
                eps_decay: 0.999,
                episodes: 500,
            },
            nn: NN::new(shape),
            create_env: env_closure,
        }
    }

    pub fn new_with(env_closure: F, shape: &[u32], episodes: usize) -> AgentNN<F, E> {
        let mut agent = Self::new(env_closure, shape);
        agent.hyper_parms.episodes = episodes;
        agent
    }

    fn get_action(&self, r: f64, env: &Box<dyn Environment<E>>, state: &Vec<f64>) -> usize {
        if r < self.hyper_parms.eps {
            env.random_action()
        } else {
            let possible = env.get_possible_actions();
            self.nn
                .run(state)
                .into_iter()
                .enumerate()
                .filter(|&(i, _)| possible[i])
                .reduce(|acc, x| if acc.1 >= x.1 { acc } else { x })
                .unwrap()
                .0
        }
    }

    fn convert_state(&self, state: E) -> Vec<f64> {
        state.into_iter().map(|x| x.try_into().unwrap()).collect()
    }

    fn predict_target(&self, reward: f64, state: &Vec<f64>) -> f64 {
        reward
            + self.hyper_parms.discount * self.nn.run(state).into_iter().reduce(f64::max).unwrap()
    }
}

impl<F> Trainable for AgentNN<F, [u16; 100]>
where
    F: Fn() -> Box<dyn Environment<[u16; 100]>>,
{
    fn train(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.hyper_parms.episodes {
            println!("Episode {i}");
            let mut env = (self.create_env)();

            let mut states = Vec::with_capacity(2);
            let mut actions: [usize; 2] = [0, 0];
            let mut rewards: [f64; 2] = [0.0, 0.0];

            states.push(self.convert_state(env.get_state()));
            actions[0] = self.get_action(rng.gen_range(0.0..1.0), &env, &states[0]);
            rewards[0] = env.step(actions[0]);

            states.push(self.convert_state(env.get_state()));
            actions[1] = self.get_action(rng.gen_range(0.0..1.0), &env, &states[1]);
            rewards[1] = env.step(actions[1]);

            let mut state = self.convert_state(env.get_state());

            let mut turn = env.get_turn();

            self.hyper_parms.eps *= self.hyper_parms.eps_decay;

            while !env.is_finished() {
                // Get predicted move form neural network for the new state
                let target1 = self.predict_target(rewards[turn], &state);

                // Adjust values to account for target value of new state
                let mut target_vec = self.nn.run(&states[turn]);
                target_vec[actions[turn]] = target1;

                actions[turn] = self.get_action(rng.gen_range(0.0..1.0), &env, &state);
                rewards[turn] = env.step(actions[turn]);

                // Train neural network on the target value with the current state
                self.nn
                    .train(&[(state, target_vec)])
                    .halt_condition(HaltCondition::Epochs(1))
                    .go();

                state = self.convert_state(env.get_state());
                turn = env.get_turn();
            }
        }
    }
}
