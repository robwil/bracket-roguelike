use crate::components::*;
use crate::map::Map;
use crate::player::player_input;
use crate::systems::VisibilitySystem;
use crate::world::new_game_state;
use rltk::{GameState, Rltk};
use specs::prelude::*;

mod components;
mod map;
mod player;
mod rect;
mod systems;
mod world;

pub struct State {
    ecs: World,
}
impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        {
            // TODO: need to understand this part of Specs better. It seems very much like an IoC container, being able to fetch by type.
            let map = self.ecs.fetch::<Map>();
            map.draw(ctx);
        }

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let gs = new_game_state();
    rltk::main_loop(context, gs)
}
