#![allow(unused_imports)]
use borderpatrol::BorderPatrol;
use ml::{AgentNN, Trainable};
use terminal_borderpatrol::display::TerminalBorderPatrol;

mod borderpatrol;
mod engine;
mod ml;
mod terminal_borderpatrol;

fn main() {
    let mut agent = AgentNN::new_with(|| Box::new(BorderPatrol::new()), &[100, 100, 100, 180], 20);
    agent.train();

    let terminal_border_patrol = Box::new(TerminalBorderPatrol::with_player_two(Box::new(agent)));
    let mut engine = engine::Engine::new(terminal_border_patrol);
    engine.start();
}
