use crate::states::MainMenuState;
use amethyst::{
    assets::Loader,
    core::{ecs::prelude::*, Time},
    input::{InputEvent, VirtualKeyCode},
    prelude::*,
    ui::{Anchor, TtfFormat, UiButton, UiButtonBuilder, UiEventType, UiText, UiTransform},
};

pub struct PausedState {
    big_text_entity: Option<Entity>,
    small_text_entity: Option<Entity>,
}

impl PausedState {
    pub fn new() -> Self {
        PausedState {
            small_text_entity: None,
            big_text_entity: None,
        }
    }
}

impl SimpleState for PausedState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        //Setup UI
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
        self.big_text_entity = Some(
            world
                .create_entity()
                .with(big_text_transform)
                .with(UiText::new(
                    font.clone(),
                    "Ready To Start".to_string(),
                    [1.0, 1.0, 1.0, 1.0],
                    50.,
                ))
                .build(),
        );

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
        self.small_text_entity = Some(
            world
                .create_entity()
                .with(small_text_transform)
                .with(UiText::new(
                    font.clone(),
                    "Press Spacebar to Start".to_string(),
                    [1.0, 1.0, 1.0, 1.0],
                    30.,
                ))
                .build(),
        );
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Input(input_event) => match input_event {
                InputEvent::KeyPressed { key_code, .. } => {
                    if key_code == VirtualKeyCode::Space {
                        return Trans::Pop;
                    }
                }
                _ => {}
            },
            _ => {}
        }

        Trans::None
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        //Delete the text entity
        data.world.delete_entity(self.big_text_entity.unwrap());
        data.world.delete_entity(self.small_text_entity.unwrap());
    }
}
