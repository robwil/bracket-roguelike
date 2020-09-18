use crate::components::*;
use crate::game::RunState;
use crate::map::*;
use crate::State;
use rltk::Point;
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    // TODO: this should clearly be a System, which could be implemented with Events like in rust-sokoban
    let mut positions = ecs.write_storage::<Position>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut player_pos = ecs.write_resource::<Point>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let players = ecs.read_storage::<Player>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();
    let entities = ecs.entities();

    for (entity, _player, pos, viewshed) in
        (&entities, &players, &mut positions, &mut viewsheds).join()
    {
        // TODO: this is a very leaky abstraction, especially requiring everything to call xy_idx
        if pos.x + delta_x < 1
            || pos.x + delta_x > map.width - 1
            || pos.y + delta_y < 1
            || pos.y + delta_y > map.height - 1
        {
            return;
        }
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        for potential_target in map.tile_content[destination_idx].iter() {
            if combat_stats.get(*potential_target).is_some() {
                // Melee Attack
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *potential_target,
                        },
                    )
                    .expect("Adding melee target failed");
                return; // So we don't move after attacking
            }
        }
        if !map.blocked[destination_idx] {
            pos.x = min(map.width - 1, max(0, pos.x + delta_x));
            pos.y = min(map.height - 1, max(0, pos.y + delta_y));
            viewshed.dirty = true;
            player_pos.x = pos.x;
            player_pos.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => return RunState::AwaitingInput, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                try_move_player(-1, 0, &mut gs.ecs)
            }

            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                try_move_player(1, 0, &mut gs.ecs)
            }

            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                try_move_player(0, -1, &mut gs.ecs)
            }

            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                try_move_player(0, 1, &mut gs.ecs)
            }

            // Diagonals
            VirtualKeyCode::Numpad9 | VirtualKeyCode::U => try_move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => try_move_player(-1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),

            _ => return RunState::AwaitingInput,
        },
    }
    RunState::PlayerTurn
}
