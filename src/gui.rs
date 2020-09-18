use crate::components::Player;
use crate::components::CombatStats;
use crate::constants::MAP_HEIGHT;
use crate::constants::MAP_WIDTH;
use rltk::{ RGB, Rltk, Console };
use specs::prelude::*;

pub fn draw_ui(ecs: &World, ctx : &mut Rltk) {
    const STATUS_BAR_TOP: i32 = MAP_HEIGHT as i32; // status bar display starts where map ends (map.height)
    ctx.draw_box(0, STATUS_BAR_TOP, MAP_WIDTH-1, 6, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        ctx.print_color(12, STATUS_BAR_TOP, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &health);
        ctx.draw_bar_horizontal(28, STATUS_BAR_TOP, 51, stats.hp, stats.max_hp, RGB::named(rltk::RED), RGB::named(rltk::BLACK));
    }

}
