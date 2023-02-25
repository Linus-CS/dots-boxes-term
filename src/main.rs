use borderpatrol::BorderPatrol;
use engine::Engine;

mod borderpatrol;
mod engine;
mod ml;

fn main() {
    let game = Box::new(BorderPatrol::new());

    // let agent = AgentNN::new(game);

    let mut engine = Engine::new(game);
    engine.start();
}
