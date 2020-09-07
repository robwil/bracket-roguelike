use crate::world::new_game_state;
use crate::player::player_input;
use crate::components::*;
use crate::map::draw_map;
use crate::map::TileType;
use rltk::{GameState, Rltk};
use specs::prelude::*;

mod components;
mod map;
mod player;
mod world;

pub struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);

        {
            // TODO: need to understand this part of Specs better. It seems very much like an IoC container, being able to fetch by type.
            let map = self.ecs.fetch::<Vec<TileType>>();
            draw_map(&map, ctx);
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
