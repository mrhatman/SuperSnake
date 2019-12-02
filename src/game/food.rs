use amethyst::core::math::Point2;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

use crate::game::defines::*;
use crate::game::Snake;

pub struct Food {
    pub pellets: HashSet<Point2<u32>>,
}

impl Food {
    pub fn new() -> Self {
        let mut pellets = HashSet::new();
        pellets.insert(Point2::new(20, 5));
        pellets.insert(Point2::new(20, 35));
        pellets.insert(Point2::new(5, 20));
        pellets.insert(Point2::new(35, 20));
        Food { pellets }
    }

    pub fn add_random_pellet(&mut self, snake: &Snake) {
        let mut rand = thread_rng();

        //Don't try to add in the screen is filled
        if snake.snake.len() + self.pellets.len() == (GRID_SIZE * GRID_SIZE) as usize {
            return;
        }

        loop {
            let new_point = Point2::new(rand.gen_range(0, GRID_SIZE), rand.gen_range(0, GRID_SIZE));

            if !self.pellets.contains(&new_point) && !snake.snake.contains(&new_point) {
                self.pellets.insert(new_point);
                break;
            }
        }
    }
}
