use crate::game::State;

mod components;
mod game;
mod map;
mod player;
mod rect;
mod systems;

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let gs = State::new();
    rltk::main_loop(context, gs)
}
