use console::Term;

pub trait Game {
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
        self.running = true;
        self.scene.init(&mut self.game);

        self.scene.render();
        while self.running {
            let key = self.scene.term.read_char().expect("Could not read key.");
            match key {
                'q' => self.running = false,
                key => {
                    let stay = self.game.react(&mut self.scene.current, key);
                    self.scene.render();
                    if !stay {
                        self.scene.undo();
                    }
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
