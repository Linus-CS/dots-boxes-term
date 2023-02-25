use borderpatrol::BorderPatrol;

mod borderpatrol;
mod engine;
mod ml;

fn main() {
    let game = Box::new(BorderPatrol::new());

    // let mut agent = ml::AgentNN::new_with(game, 200);
    // ml::Trainable::train(&mut agent);

    let mut engine = engine::Engine::new(game);
    engine.start();
}
