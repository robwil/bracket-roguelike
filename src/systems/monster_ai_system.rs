use crate::components::Monster;
use crate::components::Name;
use crate::components::Viewshed;
use rltk::{console, Point};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Monster>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, viewshed, name, monster) = data;

        for (viewshed, name, _monster) in (&viewshed, &name, &monster).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(format!("Monster {} shouts insults", name.name));
            }
        }
    }
}
