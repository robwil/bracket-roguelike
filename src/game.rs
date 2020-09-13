use crate::systems::MapIndexingSystem;
use crate::components::*;
use crate::map::Map;
use crate::player::player_input;
use crate::systems::MonsterAI;
use crate::systems::VisibilitySystem;
use rltk::Point;
use rltk::RGB;
use rltk::{GameState, Rltk};
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    pub ecs: World,
    pub run_state: RunState,
}
impl State {
    pub fn new() -> State {
        let mut gs = State {
            ecs: World::new(),
            run_state: RunState::Running,
        };
        gs.ecs.register::<Position>();
        gs.ecs.register::<Name>();
        gs.ecs.register::<CombatStats>();
        gs.ecs.register::<BlocksTile>();
        gs.ecs.register::<Renderable>();
        gs.ecs.register::<Viewshed>();
        gs.ecs.register::<Player>();
        gs.ecs.register::<Monster>();

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
                dirty: true,
            })
            .with(CombatStats{ max_hp: 30, hp: 30, defense: 2, power: 5 })
            .with(Name {
                name: "Player".to_owned(),
            })
            .with(Player {})
            .build();

        // monsters
        let mut rng = rltk::RandomNumberGenerator::new();
        for (i, room) in map.rooms.iter().skip(1).enumerate() {
            let (x, y) = room.center();
            let glyph;
            let name;
            match rng.roll_dice(1, 2) {
                1 => {
                    glyph = rltk::to_cp437('g');
                    name = "Goblin".to_owned();
                }
                _ => {
                    glyph = rltk::to_cp437('o');
                    name = "Orc".to_owned();
                }
            }
            gs.ecs
                .create_entity()
                .with(Position { x, y })
                .with(Renderable {
                    glyph,
                    fg: RGB::named(rltk::RED),
                    bg: RGB::named(rltk::BLACK),
                })
                .with(Viewshed {
                    visible_tiles: Vec::new(),
                    range: 8,
                    dirty: true,
                })
                .with(CombatStats{ max_hp: 16, hp: 16, defense: 1, power: 4 })
                .with(Name {
                    name: format!("{} #{}", &name, i),
                })
                .with(BlocksTile{})
                .with(Monster {})
                .build();
        }

        gs.ecs.insert(map);
        gs.ecs.insert(Point::new(player_x, player_y));

        gs
    }
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut monster_ai = MonsterAI {};
        monster_ai.run_now(&self.ecs);
        let mut map_indexing = MapIndexingSystem {};
        map_indexing.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if self.run_state == RunState::Running {
            self.run_systems();
            self.run_state = RunState::Paused;
        } else {
            self.run_state = player_input(self, ctx);
        }

        // TODO: need to understand this part of Specs better. It seems very much like an IoC container, being able to fetch by type.
        let map = self.ecs.fetch::<Map>();
        map.draw(ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}
