use crate::game::Snake;
use crate::states::GameState;
use amethyst::{
    core::math::Point2,
    ecs::prelude::*,
    input::{InputHandler, StringBindings, VirtualKeyCode},
};

#[derive(PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct DirectionChangeSystem {}

impl<'s> System<'s> for DirectionChangeSystem {
    type SystemData = (
        WriteExpect<'s, Snake>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, GameState>,
    );

    fn run(&mut self, (mut snake, input, game_state): Self::SystemData) {
        if *game_state == GameState::Playing {
            if input.key_is_down(VirtualKeyCode::Up) {
                let new_point = Point2::new(
                    snake.snake.get(0).unwrap().x,
                    snake.snake.get(0).unwrap().y - 1,
                );
                if new_point != *snake.snake.get(1).unwrap() {
                    snake.direction = Direction::Up;
                }
            }
            if input.key_is_down(VirtualKeyCode::Down) {
                let new_point = Point2::new(
                    snake.snake.get(0).unwrap().x,
                    snake.snake.get(0).unwrap().y + 1,
                );
                if new_point != *snake.snake.get(1).unwrap() {
                    snake.direction = Direction::Down;
                }
            }
            if input.key_is_down(VirtualKeyCode::Left) {
                let new_point = Point2::new(
                    snake.snake.get(0).unwrap().x - 1,
                    snake.snake.get(0).unwrap().y,
                );
                if new_point != *snake.snake.get(1).unwrap() {
                    snake.direction = Direction::Left;
                }
            }
            if input.key_is_down(VirtualKeyCode::Right) {
                let new_point = Point2::new(
                    snake.snake.get(0).unwrap().x + 1,
                    snake.snake.get(0).unwrap().y,
                );
                if new_point != *snake.snake.get(1).unwrap() {
                    snake.direction = Direction::Right;
                }
            }
        }
    }
}
