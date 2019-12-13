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

use crate::game::{
    defines::*, AudioHandles, DirectionChangeSystem, Food, MoveSystem, Snake, SnakeGameTile,
    UiEntities,
};
use crate::states::{GameOverState, PausedState};

use std::ops::Deref;

pub struct PrimaryState<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
    map_entity: Option<Entity>,
}

impl<'a, 'b> PrimaryState<'a, 'b> {
    pub fn new() -> Self {
        PrimaryState {
            dispatcher: None,
            map_entity: None,
        }
    }
}

impl<'a, 'b> SimpleState for PrimaryState<'a, 'b> {
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

        self.map_entity = Some(
            world
                .create_entity()
                .with(map)
                .with(Transform::from(Vector3::new(
                    TILE_SIZE as f32 / 2.0,
                    TILE_SIZE as f32 / -2.0,
                    0.0,
                )))
                .build(),
        );

        world.insert(AudioHandles { eating_noise });
        world.insert(Snake::default());
        world.insert(GameState::default());
        world.insert(Food::new());

        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(DirectionChangeSystem {}, "direction change", &[]);
        dispatcher_builder.add(MoveSystem::default(), "move system", &[]);
        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Input(input_event) => match input_event {
                InputEvent::KeyPressed { key_code, .. } => {
                    if key_code == VirtualKeyCode::P {
                        return Trans::Push(Box::new(PausedState::new()));
                    }
                }
                _ => {}
            },
            _ => {}
        }

        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        let mut state = data.world.fetch::<GameState>();
        match state.deref() {
            GameState::Playing => Trans::None,
            GameState::HitWall => {
                Trans::Switch(Box::new(GameOverState::new("You hit the wall".to_string())))
            }
            GameState::HitYourself => {
                Trans::Switch(Box::new(GameOverState::new("You hit yourself".to_string())))
            }
        }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        //Delete the text entity
        data.world
            .delete_entity(self.map_entity.unwrap())
            .expect("Failed to Delete Map");

        println!("End Primary State")
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
    Playing,
    HitYourself,
    HitWall,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Playing
    }
}
