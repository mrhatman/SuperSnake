use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{SourceHandle, WavFormat},
    core::{math::Vector3, transform::Transform},
    ecs::prelude::*,
    input::{InputEvent, VirtualKeyCode},
    prelude::*,
    renderer::{
        sprite::SpriteSheetHandle, ActiveCamera, Camera, ImageFormat, SpriteSheet,
        SpriteSheetFormat, Texture,
    },
    tiles::{MortonEncoder, TileMap},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

use crate::game::{defines::*, AudioHandles, Food, Snake, SnakeGameTile, UiEntities};

pub struct PrimaryState;

impl SimpleState for PrimaryState {
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
pub enum GameState {
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
