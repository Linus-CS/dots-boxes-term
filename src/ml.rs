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
}

pub struct AgentNN {
    hyper_parms: HyperParameters,
    nn: NN,
    env: Box<dyn Environment<[u16; 100]>>,
}

impl AgentNN {
    pub fn new(env: Box<dyn Environment<[u16; 100]>>) -> AgentNN {
        return AgentNN {
            hyper_parms: HyperParameters {
                discount: 0.95,
                eps: 0.5,
                eps_decay: 0.999,
                episodes: 500,
            },
            nn: NN::new(&[100, 100, 100, 400]),
            env,
        };
    }

    pub fn new_with(env: Box<dyn Environment<[u16; 100]>>, episodes: usize) -> AgentNN {
        let mut agent = Self::new(env);
        agent.hyper_parms.episodes = episodes;
        agent
    }
}

impl Trainable for AgentNN {
    fn train(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.hyper_parms.episodes {
            println!("Episode {i}");
            let mut state = self.env.get_state();
            self.hyper_parms.eps *= self.hyper_parms.eps_decay;
            let mut running = false;
            while !running {
                let r: f64 = rng.gen_range(0.0..1.0);
                let action = if r < self.hyper_parms.eps {
                    // Sometimes choose action randomly to create nuance
                    self.env.random_action()
                } else {
                    // Make move according to maximum output of neural network
                    let converted: Vec<f64> = state.iter().map(|x| *x as f64).collect();

                    self.nn
                        .run(&converted)
                        .into_iter()
                        .enumerate()
                        .reduce(|acc, x| if acc >= x { acc } else { x })
                        .unwrap()
                        .0
                };

                let (new_state, reward, done) = self.env.step(action);
                running = done;
                let converted: Vec<f64> = new_state.iter().map(|x| *x as f64).collect();

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
                let converted: Vec<f64> = state.iter().map(|x| *x as f64).collect();
                let mut target_vec = self.nn.run(&converted);
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
