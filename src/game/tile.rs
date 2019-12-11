use amethyst::{
    core::math::{Point2, Point3},
    ecs::prelude::*,
    tiles::Tile,
};

use crate::game::{Food, Snake, Direction};

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
                if p == snake.directions.len() -1{
                    match snake.directions[p-1]{
                        Direction::Up =>{
                            Some(12)
                        }
                        Direction::Down =>{
                            Some(13)
                        }
                        Direction::Left =>{
                            Some(14)
                        }
                        Direction::Right =>{
                            Some(15)
                        }

                    }
                }
                else{
                    match ( snake.directions.get(p.wrapping_sub(1)) ,snake.directions.get(p)){
                        (Some(Direction::Down),Some(Direction::Down)) | (Some(Direction::Up),Some(Direction::Up)) =>{
                            Some(3)
                        }
                        (Some(Direction::Right),Some(Direction::Right)) | (Some(Direction::Left),Some(Direction::Left)) =>{
                            Some(2)
                        }
                        (Some(Direction::Left),Some(Direction::Down)) | (Some(Direction::Up),Some(Direction::Right)) =>{
                            Some(6)
                        }
                       (Some(Direction::Left),Some(Direction::Up)) | (Some(Direction::Down),Some(Direction::Right)) =>{
                            Some(7)
                        }
                        (Some(Direction::Right),Some(Direction::Up)) | (Some(Direction::Down),Some(Direction::Left)) =>{
                            Some(4)
                        }
                        (Some(Direction::Right),Some(Direction::Down))| (Some(Direction::Up),Some(Direction::Left)) =>{
                            Some(5)
                        }
                        (None,Some(Direction::Down)) =>{
                            Some(8)
                        }
                       (None,Some(Direction::Up)) =>{
                            Some(9)
                        }
                       (None,Some(Direction::Right)) =>{
                            Some(10)
                        }
                       (None,Some(Direction::Left)) =>{
                            Some(11)
                        }
                       _ =>{
                            Some(1)
                        }

                    }
                }

            } else {
                let food = world.fetch::<Food>();
                if food.pellets.contains(&Point2::new(point.x, point.y)) {
                    Some(1)
                } else {
                    None
                }
            }
        }
    }

}
