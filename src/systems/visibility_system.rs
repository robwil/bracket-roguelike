use crate::Map;
use crate::Position;
use crate::Viewshed;
use rltk::field_of_view;
use rltk::Point;
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, mut viewshed, pos) = data;

        for (viewshed, pos) in (&mut viewshed, &pos).join() {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            // TODO: also a leaky abstraction here.
            // also what was the point of implementing Algorithm2D for Map and passing it into field_of_view when we still read it's width/height directly?
            viewshed
                .visible_tiles
                .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
        }
    }
}
