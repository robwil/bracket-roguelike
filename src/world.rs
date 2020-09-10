use crate::components::*;
use crate::map::*;
use crate::State;
use rltk::RGB;
use specs::prelude::*;

pub fn new_game_state() -> State {
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Player>();

    let map = Map::new_map_rooms_and_corridors();

    // entities
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
        })
        .with(Player {})
        .build();

    gs.ecs.insert(map);

    gs
}
