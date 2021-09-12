use ggez::{conf, event, Context, GameResult};
use std::path;

struct Game {}

impl event::EventHandler<ggez::GameError> for Game {
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
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = cb.build()?;
    let game = &mut Game {};
    event::run(context, event_loop, game)

    // let (ctx, event_loop) = cb.build()?;
    // let state = MainState::new()?;
    // event::run(ctx, event_loop, state)

    // // Create a game context and event loop
    // let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
    //     .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
    //     .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
    //     .add_resource_path(path::PathBuf::from("./resources"));
    //
    // let (context, event_loop) = &mut context_builder.build()?;
    // // Create the game state
    // let game = &mut Game {};
    // // Run the main event loop
    // event::run(context, event_loop, game)
}
