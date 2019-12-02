use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{output::Output, AudioBundle, Source, SourceHandle, WavFormat},
    core::{
        math::{Point2, Point3, Vector3},
        transform::{Transform, TransformBundle},
        Time,
    },
    ecs::prelude::*,
    input::{InputBundle, InputEvent, InputHandler, StringBindings, VirtualKeyCode},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::SpriteSheetHandle,
        types::DefaultBackend,
        ActiveCamera, Camera, ImageFormat, RenderingBundle, SpriteSheet, SpriteSheetFormat,
        Texture,
    },
    tiles::{MortonEncoder, RenderTiles2D, Tile, TileMap},
    ui::{Anchor, RenderUi, TtfFormat, UiBundle, UiText, UiTransform},
    utils::application_root_dir,
};
use rand::{thread_rng, Rng};
use std::collections::{HashSet, VecDeque};
use std::ops::Deref;

pub const TILE_SIZE: u32 = 32;
pub const GRID_SIZE: u32 = 40;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources_dir = app_root.join("resources\\");

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<SnakeGameTile, MortonEncoder>::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with(DirectionChangeSystem {}, "Direction Change", &[])
        .with(MoveSystem::default(), "Move system", &[]);

    let mut game = Application::new(resources_dir, SnakeState, game_data)?;
    game.run();

    Ok(())
}

struct SnakeState;

impl SimpleState for SnakeState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_camera(world);

        let tile_sprite_sheet = load_sprite_sheet(world, "Tile.png", "Tile.ron");
        let eating_noise = load_source_source(world, "EatingNoise.wav");

        let map = TileMap::<SnakeGameTile, MortonEncoder>::new(
            Vector3::new(GRID_SIZE, GRID_SIZE, 2),
            Vector3::new(TILE_SIZE, TILE_SIZE, 0),
            Some(tile_sprite_sheet),
        );

        let font = world.read_resource::<Loader>().load(
            "Poppins-Black.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

        let big_text_transform = UiTransform::new(
            "Instructions".to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.,
            60.,
            1.,
            2000.,
            50.,
        );
        let big_text = world
            .create_entity()
            .with(big_text_transform)
            .with(UiText::new(
                font.clone(),
                "Ready To Start".to_string(),
                [1.0, 1.0, 1.0, 1.0],
                50.,
            ))
            .build();

        let small_text_transform = UiTransform::new(
            "Instructions".to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.,
            -40.,
            1.,
            2000.,
            50.,
        );
        let small_text = world
            .create_entity()
            .with(small_text_transform)
            .with(UiText::new(
                font.clone(),
                "Press Spacebar to Start".to_string(),
                [1.0, 1.0, 1.0, 1.0],
                30.,
            ))
            .build();

        world
            .create_entity()
            .with(map)
            .with(Transform::from(Vector3::new(
                TILE_SIZE as f32 / 2.0,
                TILE_SIZE as f32 / -2.0,
                0.0,
            )))
            .build();

        world.insert(UiEntities {
            big_text,
            small_text,
        });
        world.insert(AudioHandles { eating_noise });
        world.insert(Snake::default());
        world.insert(GameState::default());
        world.insert(Food::new());
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        let mut state = data.world.fetch_mut::<GameState>();
        let ui_entities = data.world.fetch::<UiEntities>();
        let mut ui_texts = data.world.write_storage::<UiText>();

        match event {
            StateEvent::Input(input_event) => match input_event {
                InputEvent::KeyPressed { key_code, .. } => {
                    if (*state == GameState::WaitingToStart || *state == GameState::Paused)
                        && key_code == VirtualKeyCode::Space
                    {
                        *state = GameState::Playing;

                        ui_texts.get_mut(ui_entities.small_text).unwrap().text = "".to_string();
                        ui_texts.get_mut(ui_entities.big_text).unwrap().text = "".to_string();
                    } else if *state == GameState::Playing && key_code == VirtualKeyCode::P {
                        *state = GameState::Paused;

                        ui_texts.get_mut(ui_entities.small_text).unwrap().text =
                            "Paused".to_string();
                        ui_texts.get_mut(ui_entities.big_text).unwrap().text =
                            "Press Spacebar to Resume".to_string();
                    }
                }
                _ => {}
            },
            _ => {}
        }

        Trans::None
    }
}

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(png_path, ImageFormat::default(), (), &texture_storage)
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn load_source_source(world: &mut World, src_path: &str) -> SourceHandle {
    let loader = world.read_resource::<Loader>();
    loader.load(src_path, WavFormat, (), &world.read_resource())
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    let cam = world
        .create_entity()
        .with(Camera::standard_2d(
            (TILE_SIZE * GRID_SIZE) as f32,
            (TILE_SIZE * GRID_SIZE) as f32,
        ))
        .with(transform)
        .build();

    let mut act_cam = world.write_resource::<ActiveCamera>();
    (*act_cam).entity = Some(cam);
}

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq)]
enum GameState {
    WaitingToStart,
    Playing,
    Paused,
    GameOver,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::WaitingToStart
    }
}

struct UiEntities {
    big_text: Entity,
    small_text: Entity,
}

struct AudioHandles {
    eating_noise: SourceHandle,
}

struct Snake {
    snake: VecDeque<Point2<u32>>,
    direction: Direction,
    points_to_add: u32,
}

impl Default for Snake {
    fn default() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back(Point2::new(20, 20));
        snake.push_back(Point2::new(20, 21));
        snake.push_back(Point2::new(20, 22));
        snake.push_back(Point2::new(20, 23));
        Snake {
            snake,
            direction: Direction::Up,
            points_to_add: 0,
        }
    }
}

struct Food {
    pellets: HashSet<Point2<u32>>,
}

impl Food {
    fn new() -> Self {
        let mut pellets = HashSet::new();
        pellets.insert(Point2::new(20, 5));
        pellets.insert(Point2::new(20, 35));
        pellets.insert(Point2::new(5, 20));
        pellets.insert(Point2::new(35, 20));
        Food { pellets }
    }

    fn add_random_pellet(&mut self, snake: &Snake) {
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

#[derive(Default, Clone)]
struct SnakeGameTile;
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

struct DirectionChangeSystem {}

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

struct MoveSystem {
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
                    if snake.points_to_add > 0 {
                        snake.points_to_add -= 1;
                    } else {
                        snake.snake.pop_back();
                    }
                }
            }
        }
    }
}

fn play_eat_sound(
    audio_handles: &AudioHandles,
    storage: &AssetStorage<Source>,
    output: Option<&Output>,
) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&audio_handles.eating_noise) {
            output.play_once(sound, 1.0);
        }
    }
}
