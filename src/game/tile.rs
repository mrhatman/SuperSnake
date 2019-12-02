use amethyst::{
    core::math::{Point2, Point3},
    ecs::prelude::*,
    tiles::Tile,
};

use crate::game::{Food, Snake};

#[derive(Default, Clone)]
pub struct SnakeGameTile;
impl Tile for SnakeGameTile {
    fn sprite(&self, point: Point3<u32>, world: &World) -> Option<usize> {
        if point.z == 0 {
            Some(0)
        } else {
            let snake = world.fetch::<Snake>();

            let pos = snake
                .snake
                .iter()
                .enumerate()
                .find(|&(_, p)| p.x == point.x && p.y == point.y)
                .map(|(loc, _)| loc);

            if let Some(p) = pos {
                if p == 0 {
                    Some(1)
                } else {
                    Some(2)
                }
            } else {
                let food = world.fetch::<Food>();
                if food.pellets.contains(&Point2::new(point.x, point.y)) {
                    Some(3)
                } else {
                    None
                }
            }
        }
    }
}
