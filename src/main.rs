#![allow(unused_imports)]
use ai::AdvancedPlayer;
use borderpatrol::BorderPatrol;
use engine::Engine;
use ml::{BorderPatrolAgent, HyperParameters};
use terminal_borderpatrol::display::TerminalBorderPatrol;

mod ai;
mod borderpatrol;
mod engine;
mod ml;
mod terminal_borderpatrol;

fn main() {
    let game = TerminalBorderPatrol::new();
    let mut engine = Engine::new(Box::new(game));

    engine.start();
}

fn train_model(save_at: &str) {
    let mut agent = BorderPatrolAgent::new_with(
        &[100, 300, 300, 200],
        HyperParameters {
            discount: 0.95,
            eps: 0.4,
            eps_decay: 0.999,
            episodes: 20000,
        },
    );
    agent.train();
    agent.save(save_at);
}

fn play_against(model_at: &str) {
    let agent = BorderPatrolAgent::from_file(model_at);

    let terminal_border_patrol = Box::new(TerminalBorderPatrol::with_player_two(Box::new(agent)));
    let mut engine = engine::Engine::new(terminal_border_patrol);
    engine.start();
}
