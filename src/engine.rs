#![allow(unused)]
use console::Term;

pub trait Game {
    fn update(&mut self, content: &mut String) -> bool;
    fn wait_for_input(&self) -> bool;
    fn init_screen(&mut self) -> String;
    fn react(&mut self, content: &mut String, key: char) -> bool;
}

pub struct Engine {
    game: Box<dyn Game>,
    scene: Scene,
    running: bool,
}

impl Engine {
    pub fn new(game: Box<dyn Game>) -> Engine {
        Engine {
            game,
            scene: Scene::new(),
            running: false,
        }
    }

    pub fn start(&mut self) {
        self.scene.term.set_title("Terminal Engine");
        self.running = true;
        self.scene.init(&mut self.game);

        self.scene.render();
        while self.running {
            let key = if self.game.wait_for_input() {
                Some(self.scene.term.read_char().expect("Could not read key."))
            } else {
                None
            };

            match key {
                Some('q') => self.running = false,
                Some(key) => {
                    let stay = self.game.react(&mut self.scene.current, key);
                    self.scene.render();
                    if !stay {
                        self.scene.undo();
                    }
                }
                _ => {
                    self.running = self.game.update(&mut self.scene.current);
                    self.scene.render();
                }
            }
        }
        self.scene.term.clear_screen().unwrap();
    }
}

struct Scene {
    current: String,
    tmp: String,
    last: String,
    term: Term,
}

impl Scene {
    fn new() -> Scene {
        Scene {
            current: String::new(),
            tmp: String::new(),
            last: String::new(),
            term: Term::stdout(),
        }
    }

    fn init(&mut self, game: &mut Box<dyn Game>) {
        self.current = game.init_screen();
        self.last = self.current.to_owned();
    }

    fn undo(&mut self) {
        self.current = self.last.to_owned();
        self.tmp = self.current.to_owned();
    }

    pub fn render(&mut self) {
        self.term.clear_screen().unwrap();
        println!("{}", self.current);
        self.last = self.tmp.to_owned();
        self.tmp = self.current.to_owned();
    }
}
