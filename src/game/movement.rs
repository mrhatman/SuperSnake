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
        ReadExpect<'s, UiEntities>,
        WriteStorage<'s, UiText>,
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
            ui_entities,
            mut ui_texts,
            sources,
            audio_handles,
            audio_output,
        ): Self::SystemData,
    ) {
        if *game_state == GameState::Playing {
            self.time_remainder_sec += time.delta_seconds();

            if self.time_remainder_sec > 0.05 {
                self.time_remainder_sec -= 0.05;

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
                    *game_state = GameState::GameOver;
                    ui_texts.get_mut(ui_entities.small_text).unwrap().text =
                        "You hit the edge".to_string();
                    ui_texts.get_mut(ui_entities.big_text).unwrap().text = "Game Over".to_string();
                } else if snake.snake.contains(&new_point) {
                    *game_state = GameState::GameOver;
                    ui_texts.get_mut(ui_entities.small_text).unwrap().text =
                        "You hit yourself".to_string();
                    ui_texts.get_mut(ui_entities.big_text).unwrap().text = "Game Over".to_string();
                } else {
                    if food.pellets.remove(&new_point) {
                        food.add_random_pellet(&snake);
                        snake.points_to_add += 1;
                        play_eat_sound(
                            &audio_handles,
                            &sources,
                            audio_output.as_ref().map(|o| o.deref()),
                        );
                    }
                    snake.snake.push_front(new_point);
                    let direction = snake.direction.clone();
                    snake.directions.push_front(direction);


                    if snake.points_to_add > 0 {
                        snake.points_to_add -= 1;
                    } else {
                        snake.snake.pop_back();
                        snake.directions.pop_back();
                    }
                }
            }
        }
    }
}
