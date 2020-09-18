use crate::game::State;

mod components;
mod constants;
mod game;
mod gui;
mod map;
mod player;
mod rect;
mod systems;

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    context.with_post_scanlines(true);
    let gs = State::new();
    rltk::main_loop(context, gs)
}
