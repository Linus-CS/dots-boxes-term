use nn::{HaltCondition, NN};
use rand::Rng;

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
    fn step(&mut self, action: usize) -> (T, f64, bool);
    fn get_state(&self) -> T;
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

impl<F, E> AgentNN<F, E>
where
    F: Fn() -> Box<dyn Environment<E>>,
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
}

impl<C, F, E> Trainable for AgentNN<F, E>
where
    C: Into<f64>,
    F: Fn() -> Box<dyn Environment<E>>,
    E: IntoIterator<Item = C> + Clone,
{
    fn train(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.hyper_parms.episodes {
            println!("Episode {i}");

            let mut env = (self.create_env)();
            let mut state = env.get_state();
            self.hyper_parms.eps *= self.hyper_parms.eps_decay;
            let mut running = false;

            while !running {
                let r: f64 = rng.gen_range(0.0..1.0);
                let converted_state: Vec<f64> = state
                    .clone()
                    .into_iter()
                    .map(|x| x.try_into().unwrap())
                    .collect();

                let action = if r < self.hyper_parms.eps {
                    // Sometimes choose action randomly to create nuance
                    env.random_action()
                } else {
                    // Make move according to maximum output of neural network
                    let possible = env.get_possible_actions();
                    self.nn
                        .run(&converted_state)
                        .into_iter()
                        .enumerate()
                        .reduce(|acc, x| {
                            if acc.1 >= x.1 && possible[x.0] {
                                acc
                            } else {
                                x
                            }
                        })
                        .unwrap()
                        .0
                };

                let (new_state, reward, done) = env.step(action);
                running = done;
                let converted: Vec<f64> = new_state
                    .clone()
                    .into_iter()
                    .map(|x| x.try_into().unwrap())
                    .collect();

                // Get predicted move form neural network for the new state
                let target = reward
                    + self.hyper_parms.discount
                        * self
                            .nn
                            .run(&converted)
                            .into_iter()
                            .reduce(f64::max)
                            .unwrap();

                // Adjust values to account for target value of new state
                let mut target_vec = self.nn.run(&converted_state);
                target_vec[action] = target;

                // Train neural network on the target value with the current state
                self.nn
                    .train(&[(converted, target_vec)])
                    .halt_condition(HaltCondition::Epochs(1))
                    .go();

                state = new_state;
            }
        }
    }
}
