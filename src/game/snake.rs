use crate::game::Direction;
use amethyst::core::math::Point2;
use std::collections::VecDeque;


pub struct Snake {
    pub snake: VecDeque<Point2<u32>>,
    pub directions: VecDeque<Direction>,
    pub direction: Direction,
}

impl Default for Snake {
    fn default() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back(Point2::new(20, 20));
        snake.push_back(Point2::new(20, 21));
        snake.push_back(Point2::new(20, 22));
        snake.push_back(Point2::new(20, 23));

        let mut directions = VecDeque::new();
        directions.push_back(Direction::Up);
        directions.push_back(Direction::Up);
        directions.push_back(Direction::Up);
        directions.push_back(Direction::Up);
        Snake {
            snake,
            directions,
            direction: Direction::Up,
        }
    }
}
