use crate::states::MainMenuState;
use amethyst::{
    assets::Loader,
    core::ecs::prelude::*,
    prelude::*,
    ui::{Anchor, TtfFormat, UiButton, UiButtonBuilder, UiEventType, UiText, UiTransform},
};

pub struct SettingsState {
    text_entity: Option<Entity>,
    exit_button_entity: Option<UiButton>,
}

impl SettingsState {
    pub fn new() -> Self {
        SettingsState {
            text_entity: None,
            exit_button_entity: None,
        }
    }
}

impl SimpleState for SettingsState {
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
            "Settings Text".to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.,
            500.,
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
                    "Settings State".to_string(),
                    [1.0, 1.0, 1.0, 1.0],
                    50.,
                ))
                .build(),
        );

        self.exit_button_entity = Some(
            UiButtonBuilder::<(), u32>::new("Exit")
                .with_size(200.0, 36.0)
                .with_anchor(Anchor::Middle)
                .with_font(font.clone())
                .with_id(0)
                .with_position(0.0, 200.0)
                .with_font_size(24.0f32)
                .with_text_color([1.0f32, 1.0, 1.0, 1.0])
                .with_hover_text_color([1.0f32, 0.0f32, 0.0f32, 1.0f32])
                .build_from_world(&world)
                .1,
        );
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Ui(ui_event) => {
                if ui_event.event_type == UiEventType::ClickStart {
                    if ui_event.target == self.exit_button_entity.as_ref().unwrap().image_entity {
                        return Trans::Switch(Box::new(MainMenuState::new()));
                    }
                }
            }
            _ => {}
        }

        Trans::None
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        //Delete the text entity
        data.world
            .delete_entity(self.text_entity.unwrap())
            .expect("Failed to Delete Entity");
        data.world
            .delete_entity(self.exit_button_entity.as_ref().unwrap().text_entity)
            .expect("Failed to Delete Entity");
        data.world
            .delete_entity(self.exit_button_entity.as_ref().unwrap().image_entity)
            .expect("Failed to Delete Entity");
    }
}
