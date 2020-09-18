use crate::components::CombatStats;
use crate::components::Name;
use crate::components::Player;
use crate::components::Position;
use crate::constants::MAP_HEIGHT;
use crate::constants::MAP_WIDTH;
use crate::game::GameLog;
use crate::map::Map;
use rltk::Point;
use rltk::{Rltk, RGB};
use specs::prelude::*;

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    const STATUS_BAR_TOP: usize = MAP_HEIGHT; // status bar display starts where map ends (map.height)
    ctx.draw_box(
        0,
        STATUS_BAR_TOP,
        MAP_WIDTH - 1,
        6,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        ctx.print_color(
            12,
            STATUS_BAR_TOP,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            &health,
        );
        ctx.draw_bar_horizontal(
            28,
            STATUS_BAR_TOP,
            51,
            stats.hp,
            stats.max_hp,
            RGB::named(rltk::RED),
            RGB::named(rltk::BLACK),
        );
    }

    let log = ecs.fetch::<GameLog>();
    let y = STATUS_BAR_TOP + 1;
    for (i, s) in log.entries.iter().rev().enumerate() {
        ctx.print(2, y + i, s);
        if i == 4 {
            // can only fit 5 log items in our 6-height tall box
            break;
        }
    }

    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));
    draw_tooltips(ecs, ctx);
}

fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height {
        return;
    }
    let mut tooltip: Vec<String> = Vec::new();
    for (name, position) in (&names, &positions).join() {
        let idx = map.xy_idx(position.x, position.y);
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.visible_tiles[idx] {
            tooltip.push(name.name.to_string());
        }
    }

    // draw a tooltip, like "<- name" or "name ->" depending on where mouse is
    if !tooltip.is_empty() {
        let max_width_plus_buffer = (tooltip
            .iter()
            .map(|s| s.len())
            .max_by(|x, y| x.cmp(y))
            .unwrap()
            + 3) as i32;
        let mut mouse_offset = 1;
        let mut left_x = mouse_pos.0 + 4;
        let mut arrow = "<-";
        if mouse_pos.0 > 40 {
            mouse_offset = -2;
            left_x = mouse_pos.0 - max_width_plus_buffer;
            arrow = "->";
        }
        let arrow_pos = Point::new(mouse_pos.0 + mouse_offset, mouse_pos.1);
        let mut y = mouse_pos.1;
        for s in tooltip.iter() {
            ctx.print_color(
                left_x,
                y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::GREY),
                s,
            );
            let padding = (max_width_plus_buffer - s.len() as i32) - 1;
            for i in 0..padding {
                let mut x = arrow_pos.x + 1 + i;
                if mouse_pos.0 > 40 {
                    x = arrow_pos.x - i;
                }
                ctx.print_color(
                    x,
                    y,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::GREY),
                    &" ".to_string(),
                );
            }
            y += 1;
        }
        ctx.print_color(
            arrow_pos.x,
            arrow_pos.y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::GREY),
            arrow,
        );
    }
}
