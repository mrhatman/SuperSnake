use amethyst::{
    assets::Loader,
    core::{ecs::prelude::*, Time},
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

use crate::states::MainMenuState;

pub struct SplashState {
    remaining_time: f32,
    text_entity: Option<Entity>,
}

impl SplashState {
    pub fn new() -> Self {
        SplashState {
            remaining_time: 2.0,
            text_entity: None,
        }
    }
}

impl SimpleState for SplashState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        //Setup UI
        let font = world.read_resource::<Loader>().load(
            "Poppins-Black.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

        let text_transform = UiTransform::new(
            "Amethyst Text".to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.,
            60.,
            1.,
            2000.,
            50.,
        );
        self.text_entity = Some(
            world
                .create_entity()
                .with(text_transform)
                .with(UiText::new(
                    font.clone(),
                    "Powered by Amethyst".to_string(),
                    [1.0, 1.0, 1.0, 1.0],
                    50.,
                ))
                .build(),
        );
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let elapsed_time = data.world.fetch::<Time>().delta_seconds();
        self.remaining_time -= elapsed_time;

        if self.remaining_time > 0.0 {
            //Fade the Text
            let fade_value = self.remaining_time.min(1.0) / 1.0;
            let mut ui_texts = data.world.write_storage::<UiText>();
            let text = ui_texts.get_mut(self.text_entity.unwrap()).unwrap();
            text.color[0] = 1.0 * fade_value;
            text.color[1] = 1.0 * fade_value;
            text.color[2] = 1.0 * fade_value;
            Trans::None
        } else {
            Trans::Switch(Box::new(MainMenuState::new()))
        }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        //Delete the text entity
        data.world
            .delete_entity(self.text_entity.unwrap())
            .expect("Failed to Delete Entity");
    }
}
