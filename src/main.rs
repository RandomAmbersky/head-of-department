use ggez::{conf, event, Context, GameResult};
use std::path;

struct MyGame {}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl event::EventHandler<ggez::GameError> for MyGame {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // TODO: update game logic here
        Ok(())
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        // TODO: update draw here
        Ok(())
    }
}

pub fn main() -> GameResult {

    let cb = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(3000.0, 2000.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (mut context, event_loop) = cb.build()?;
    let game = MyGame::new(&mut context);
    event::run(context, event_loop, game)

}
