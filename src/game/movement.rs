use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{math::Point2, Time},
    ecs::prelude::*,
    ui::UiText,
};

use crate::game::{defines::*, play_eat_sound, AudioHandles, Direction, Food, Snake, UiEntities};
use crate::states::GameState;

use std::ops::Deref;

pub struct MoveSystem {
    time_remainder_sec: f32,
}

impl Default for MoveSystem {
    fn default() -> Self {
        Self {
            time_remainder_sec: 0.0,
        }
    }
}

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteExpect<'s, Snake>,
        WriteExpect<'s, Food>,
        Read<'s, Time>,
        Write<'s, GameState>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, AudioHandles>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (
            mut snake,
            mut food,
            time,
            mut game_state,
            sources,
            audio_handles,
            audio_output,
        ): Self::SystemData,
    ) {
        let mut points_to_add = 0;
        self.time_remainder_sec += time.delta_seconds();

        if self.time_remainder_sec > MOVEMENT_PERIOD {
            self.time_remainder_sec -= MOVEMENT_PERIOD;

            //Move snake
            let new_point = match snake.direction {
                Direction::Up => Point2::new(
                    snake.snake.get(0).unwrap().x,
                    snake.snake.get(0).unwrap().y.overflowing_sub(1).0,
                ),
                Direction::Down => Point2::new(
                    snake.snake.get(0).unwrap().x,
                    snake.snake.get(0).unwrap().y + 1,
                ),
                Direction::Left => Point2::new(
                    snake.snake.get(0).unwrap().x.overflowing_sub(1).0,
                    snake.snake.get(0).unwrap().y,
                ),
                Direction::Right => Point2::new(
                    snake.snake.get(0).unwrap().x + 1,
                    snake.snake.get(0).unwrap().y,
                ),
            };
            if new_point.x >= GRID_SIZE || new_point.y >= GRID_SIZE {
                *game_state = GameState::HitWall;
            } else if snake.snake.contains(&new_point) {
                *game_state = GameState::HitYourself;
            } else {
                if food.pellets.remove(&new_point) {
                    food.add_random_pellet(&snake);
                    points_to_add += 1;
                    play_eat_sound(
                        &audio_handles,
                        &sources,
                        audio_output.as_ref().map(|o| o.deref()),
                    );
                }
                snake.snake.push_front(new_point);
                let direction = snake.direction.clone();
                snake.directions.push_front(direction);

                if points_to_add == 0 {
                    snake.snake.pop_back();
                    snake.directions.pop_back();
                }
            }
        }
    }
}
