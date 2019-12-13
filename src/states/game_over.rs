use crate::states::MainMenuState;
use amethyst::{
    assets::Loader,
    core::ecs::prelude::*,
    input::InputEvent,
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub struct GameOverState {
    reason_text: String,
    big_text_entity: Option<Entity>,
    small_text_entity: Option<Entity>,
}

impl GameOverState {
    pub fn new(reason_text: String) -> Self {
        GameOverState {
            reason_text,
            small_text_entity: None,
            big_text_entity: None,
        }
    }
}

impl SimpleState for GameOverState {
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
            "Big Text".to_string(),
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
                    "Game Over".to_string(),
                    [1.0, 1.0, 1.0, 1.0],
                    50.,
                ))
                .build(),
        );

        let small_text_transform = UiTransform::new(
            "Small Text".to_string(),
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
                    self.reason_text.clone() + " - Press Any Key to Continue",
                    [1.0, 1.0, 1.0, 1.0],
                    30.,
                ))
                .build(),
        );
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Input(input_event) => match input_event {
                InputEvent::KeyPressed { .. } => {
                    return Trans::Switch(Box::new(MainMenuState::new()));
                }
                _ => {}
            },
            _ => {}
        }

        Trans::None
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        //Delete the text entity
        data.world
            .delete_entity(self.big_text_entity.unwrap())
            .expect("Failed to Delete Entity");
        data.world
            .delete_entity(self.small_text_entity.unwrap())
            .expect("Failed to Delete Entity");
    }
}
