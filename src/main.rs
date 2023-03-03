#![allow(unused_imports)]
use borderpatrol::BorderPatrol;
use ml::BorderPatrolAgent;
use terminal_borderpatrol::display::TerminalBorderPatrol;

mod borderpatrol;
mod engine;
mod ml;
mod terminal_borderpatrol;

fn main() {
    let mut agent = BorderPatrolAgent::new_with(&[100, 100, 100, 200], 20);
    agent.train();
    agent.save("model.json");

    // let terminal_border_patrol = Box::new(TerminalBorderPatrol::with_player_two(Box::new(agent)));
    // let mut engine = engine::Engine::new(terminal_border_patrol);
    // engine.start();
}
